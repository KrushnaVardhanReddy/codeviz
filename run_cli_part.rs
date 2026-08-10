        }
    }

    if args.len() > 1 && (args[1] == "run" || args[1] == "check") {
        let is_check = args[1] == "check";
        let run_args = parse_run_args(&args[2..])?;

        if is_check && run_args.format != OutputFormat::Mermaid {
            return Err("check command only supports Mermaid output format".to_string());
        }

        let mut registry = LanguageRegistry::new();
        registry.register(Box::new(PythonParser::new()));
        registry.register(Box::new(TypeScriptParser::new()));
        registry.register(Box::new(GoParser::new()));
        registry.register(Box::new(RustLangParser::new()));
        registry.register(Box::new(JavaParser::new()));
        registry.register(Box::new(KotlinParser::new()));

        let config = load_config()?;
        let cache_dir = Path::new(&config.cache.dir);
        let cache_enabled = config.cache.enabled && !run_args.no_cache;
        let manager = codeviz_core::CacheManager::new(cache_dir, env!("CARGO_PKG_VERSION"));

        let config_path = run_args.config_path.clone().unwrap_or_else(|| "codeviz.toml".to_string());

        if cache_enabled {
            let meta_path = cache_dir.join("meta.json");
            let mut invalidate_cache = false;

            let config_mtime = if let Ok(metadata) = std::fs::metadata(&config_path) {
                metadata.modified().ok().and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok()).map(|d| d.as_nanos())
            } else {
                None
            };

            #[derive(serde::Serialize, serde::Deserialize)]
            struct CacheMeta {
                config_mtime: Option<u128>,
                version: String,
            }

            if let Ok(meta_str) = std::fs::read_to_string(&meta_path) {
                if let Ok(meta) = serde_json::from_str::<CacheMeta>(&meta_str) {
                    if meta.version != env!("CARGO_PKG_VERSION") || meta.config_mtime != config_mtime {
                        invalidate_cache = true;
                    }
                } else {
                    invalidate_cache = true;
                }
            } else {
                invalidate_cache = true;
            }

            if invalidate_cache {
                let _ = manager.clear();
                if std::fs::create_dir_all(cache_dir).is_ok() {
                    let new_meta = CacheMeta {
                        config_mtime,
                        version: env!("CARGO_PKG_VERSION").to_string(),
                    };
                    if let Ok(meta_str) = serde_json::to_string(&new_meta) {
                        let _ = std::fs::write(meta_path, meta_str);
                    }
                }
            }
        }

        let mut files = walk_dir(Path::new(&run_args.path))?;
        files.retain(|f| !f.starts_with(cache_dir));

        let mut merged_graph = CodeGraph::new(GraphMeta {
            language: "mixed".to_string(),
            source_root: run_args.path.clone(),
            generated_at: "2024-01-01T00:00:00Z".to_string(), // In a real app, use chrono
            node_count: 0,
            edge_count: 0,
        });

        let mut parsed_count = 0;

        for file in files {
            // Check if file extension is supported before reading source
            if let Some(ext) = file.extension().and_then(|e| e.to_str())
                && registry.find_parser(ext).is_some()
            {
                if cache_enabled
                    && let Some(entry) = manager.get(&file)
                {
                    merged_graph.nodes.extend(entry.nodes);
                    merged_graph.edges.extend(entry.edges);
                    continue;
                }

                if let Ok(source) = std::fs::read_to_string(&file)
                    && let Ok(graph) = registry.parse_file(&file.to_string_lossy(), &source)
                {
                    merged_graph.nodes.extend(graph.nodes.clone());
                    merged_graph.edges.extend(graph.edges.clone());
                    parsed_count += 1;

                    if cache_enabled {
                        let _ = manager.put(&file, graph.nodes, graph.edges);
                    }
                }
            }
        }

        merged_graph.meta.node_count = merged_graph.nodes.len();
        merged_graph.meta.edge_count = merged_graph.edges.len();

        prune_graph(&mut merged_graph, run_args.depth);

        let mut architecture_violations = false;
        if is_check {
            let config = match &run_args.config_path {
                Some(path) => codeviz_core::Config::load_from_file(std::path::Path::new(path)).unwrap_or_default(),
                None => codeviz_core::Config::load_from_dir(&std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."))).unwrap_or_default(),
            };
            for edge in &merged_graph.edges {
                if edge.kind == codeviz_core::EdgeKind::Imports {
                    for rule in &config.architecture.rules {
                        if edge.from_id.contains(&rule.from) && edge.to_id.contains(&rule.cannot_import) {
                            println!(
                                "❌ Architectural violation: {} cannot import {}",
                                edge.from_id, edge.to_id
                            );
                            architecture_violations = true;
                        }
                    }
                }
            }
        }

        match run_args.format {
            OutputFormat::Json => match render_json(&merged_graph) {
                Ok(json_str) => println!("{}", json_str),
                Err(e) => return Err(e),
            },
            OutputFormat::Dot => {
                let dot_str = render_dot(&merged_graph);
                println!("{}", dot_str);
            }
            OutputFormat::Mermaid => {
                let outputs = if let Some(out_file) = run_args.output {
                    let d_str = match run_args.diagram {
                        DiagramKind::ModuleGraph => "module",
                        DiagramKind::CallGraph => "call",
                        DiagramKind::ClassDiagram => "class",
                    };
                    vec![OutputTarget {
                        file: out_file,
                        diagram_type: d_str.to_string(),
                    }]
                } else {
                    let config = load_config()?;
                    config.outputs
                };

                let renderer = MermaidRenderer::new();
                let mut all_up_to_date = !architecture_violations;

                for target in outputs {
                    let diagram_kind = parse_diagram_kind(&target.diagram_type)?;
                    let mermaid_diagram = renderer.render(&merged_graph, diagram_kind);

                    let markdown = match std::fs::read_to_string(&target.file) {
                        Ok(c) => c,
                        Err(e) => {
                            eprintln!("Failed to read output file {}: {}", target.file, e);
                            if is_check {
                                all_up_to_date = false;
                            }
                            continue;
                        }
                    };

                    if is_check {
                        match check_diagram_up_to_date(&target.file, &markdown, &mermaid_diagram) {
                            Ok(true) => {}
                            Ok(false) => {
                                all_up_to_date = false;
                            }
                            Err(e) => {
                                eprintln!("Error checking diagram in {}: {}", target.file, e);
                                all_up_to_date = false;
                            }
                        }
                    } else {
                        let updated_markdown = match inject_mermaid(&markdown, &mermaid_diagram) {
                            Ok(c) => c,
                            Err(e) => {
                                eprintln!("Failed to inject mermaid into {}: {}", target.file, e);
                                continue;
                            }
                        };

                        if let Err(e) = std::fs::write(&target.file, updated_markdown) {
                            eprintln!("Failed to write output file {}: {}", target.file, e);
                            continue;
                        }

                        println!(
                            "Successfully parsed {} files, generated diagram with {} nodes and {} edges. Output: {}",
                            parsed_count,
                            merged_graph.meta.node_count,
                            merged_graph.meta.edge_count,
                            target.file
                        );
                    }
                }

                if is_check {
                    return Ok(all_up_to_date);
                }
            }
        }

        return Ok(true);
    }

    // Default error for unknown subcommands
    Err("Unknown command or options. Try --help".to_string())
}
