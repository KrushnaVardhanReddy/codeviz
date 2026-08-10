use codeviz_core::ir::{CfgBlock, CfgBlockKind, CfgEdge, CfgEdgeKind, ControlFlowGraph};
use codeviz_core::parser::ParseError;
use tree_sitter::Node as TsNode;

/// Builds a control flow graph for the given function node.
pub fn build_cfg(
    func_node: TsNode,
    source_bytes: &[u8],
    func_id: &str,
) -> Result<ControlFlowGraph, ParseError> {
    let mut blocks = Vec::new();
    let mut edges = Vec::new();
    let mut block_id_counter = 0;

    let mut get_next_id = || {
        let id = format!("{}_b{}", func_id, block_id_counter);
        block_id_counter += 1;
        id
    };

    let entry_id = get_next_id();
    blocks.push(CfgBlock {
        id: entry_id.clone(),
        kind: CfgBlockKind::Entry,
        label: "Entry".to_string(),
        line: Some(func_node.start_position().row as u32 + 1),
    });

    let exit_id = get_next_id();
    blocks.push(CfgBlock {
        id: exit_id.clone(),
        kind: CfgBlockKind::Exit,
        label: "Exit".to_string(),
        line: Some(func_node.end_position().row as u32 + 1),
    });

    let body = func_node.child_by_field_name("body");
    if let Some(body_node) = body {
        let last_id = traverse_statements(
            body_node,
            source_bytes,
            entry_id,
            &exit_id,
            &exit_id,
            None,
            &mut blocks,
            &mut edges,
            &mut get_next_id,
        )?;

        edges.push(CfgEdge {
            from_id: last_id,
            to_id: exit_id.clone(),
            kind: CfgEdgeKind::Normal,
            label: None,
        });
    } else {
        edges.push(CfgEdge {
            from_id: entry_id.clone(),
            to_id: exit_id.clone(),
            kind: CfgEdgeKind::Normal,
            label: None,
        });
    }

    Ok(ControlFlowGraph {
        function_id: func_id.to_string(),
        blocks,
        cfg_edges: edges,
    })
}

#[allow(clippy::too_many_arguments)]
fn traverse_statements(
    node: TsNode,
    source_bytes: &[u8],
    mut current_id: String,
    exit_id: &str,
    loop_exit_id: &str,
    loop_header_id: Option<&str>,
    blocks: &mut Vec<CfgBlock>,
    edges: &mut Vec<CfgEdge>,
    get_next_id: &mut dyn FnMut() -> String,
) -> Result<String, ParseError> {
    let mut cursor = node.walk();
    let children: Vec<TsNode> = node.children(&mut cursor).collect();

    // If it's a block statement, process children
    if node.kind() == "block" || node.kind() == "module" {
        for child in children {
            current_id = traverse_statements(
                child,
                source_bytes,
                current_id,
                exit_id,
                loop_exit_id,
                loop_header_id,
                blocks,
                edges,
                get_next_id,
            )?;
        }
        return Ok(current_id);
    }

    let line = Some(node.start_position().row as u32 + 1);

    match node.kind() {
        "if_statement" => {
            let cond_node = node.child_by_field_name("condition");
            let cond_label = if let Some(c) = cond_node {
                c.utf8_text(source_bytes).unwrap_or("condition").to_string()
            } else {
                "condition".to_string()
            };

            let cond_id = get_next_id();
            blocks.push(CfgBlock {
                id: cond_id.clone(),
                kind: CfgBlockKind::Condition,
                label: cond_label,
                line,
            });
            edges.push(CfgEdge {
                from_id: current_id,
                to_id: cond_id.clone(),
                kind: CfgEdgeKind::Normal,
                label: None,
            });

            let merge_id = get_next_id();
            blocks.push(CfgBlock {
                id: merge_id.clone(),
                kind: CfgBlockKind::Block,
                label: "merge".to_string(),
                line: None,
            });

            // True branch
            let cons_node = node.child_by_field_name("consequence");
            if let Some(cons) = cons_node {
                let mut true_branch_id = get_next_id();
                blocks.push(CfgBlock {
                    id: true_branch_id.clone(),
                    kind: CfgBlockKind::Block,
                    label: "then".to_string(),
                    line,
                });
                edges.push(CfgEdge {
                    from_id: cond_id.clone(),
                    to_id: true_branch_id.clone(),
                    kind: CfgEdgeKind::TrueBranch,
                    label: Some("✓ true".to_string()),
                });
                true_branch_id = traverse_statements(
                    cons,
                    source_bytes,
                    true_branch_id,
                    exit_id,
                    loop_exit_id,
                    loop_header_id,
                    blocks,
                    edges,
                    get_next_id,
                )?;
                edges.push(CfgEdge {
                    from_id: true_branch_id,
                    to_id: merge_id.clone(),
                    kind: CfgEdgeKind::Normal,
                    label: None,
                });
            }

            // False branch (elif / else)
            // In python, an if_statement can have multiple alternative/elif nodes and one else block.
            let mut walk = node.walk();
            let alt_nodes = node.children_by_field_name("alternative", &mut walk);
            let mut curr_cond_id = cond_id;

            let mut handled_else = false;
            for alt in alt_nodes {
                if alt.kind() == "elif_clause" {
                    let elif_cond = alt.child_by_field_name("condition");
                    let elif_cond_label = if let Some(c) = elif_cond {
                        c.utf8_text(source_bytes).unwrap_or("condition").to_string()
                    } else {
                        "condition".to_string()
                    };
                    let elif_cond_id = get_next_id();
                    blocks.push(CfgBlock {
                        id: elif_cond_id.clone(),
                        kind: CfgBlockKind::Condition,
                        label: elif_cond_label,
                        line: Some(alt.start_position().row as u32 + 1),
                    });
                    edges.push(CfgEdge {
                        from_id: curr_cond_id.clone(),
                        to_id: elif_cond_id.clone(),
                        kind: CfgEdgeKind::FalseBranch,
                        label: Some("✗ false".to_string()),
                    });
                    curr_cond_id = elif_cond_id.clone();

                    if let Some(cons) = alt.child_by_field_name("consequence") {
                        let mut true_branch_id = get_next_id();
                        blocks.push(CfgBlock {
                            id: true_branch_id.clone(),
                            kind: CfgBlockKind::Block,
                            label: "elif".to_string(),
                            line: Some(alt.start_position().row as u32 + 1),
                        });
                        edges.push(CfgEdge {
                            from_id: elif_cond_id.clone(),
                            to_id: true_branch_id.clone(),
                            kind: CfgEdgeKind::TrueBranch,
                            label: Some("✓ true".to_string()),
                        });
                        true_branch_id = traverse_statements(
                            cons,
                            source_bytes,
                            true_branch_id,
                            exit_id,
                            loop_exit_id,
                            loop_header_id,
                            blocks,
                            edges,
                            get_next_id,
                        )?;
                        edges.push(CfgEdge {
                            from_id: true_branch_id,
                            to_id: merge_id.clone(),
                            kind: CfgEdgeKind::Normal,
                            label: None,
                        });
                    }
                } else if alt.kind() == "else_clause" {
                    handled_else = true;
                    let mut false_branch_id = get_next_id();
                    blocks.push(CfgBlock {
                        id: false_branch_id.clone(),
                        kind: CfgBlockKind::Block,
                        label: "else".to_string(),
                        line: Some(alt.start_position().row as u32 + 1),
                    });
                    edges.push(CfgEdge {
                        from_id: curr_cond_id.clone(),
                        to_id: false_branch_id.clone(),
                        kind: CfgEdgeKind::FalseBranch,
                        label: Some("✗ false".to_string()),
                    });
                    if let Some(body) = alt.child_by_field_name("body") {
                        false_branch_id = traverse_statements(
                            body,
                            source_bytes,
                            false_branch_id,
                            exit_id,
                            loop_exit_id,
                            loop_header_id,
                            blocks,
                            edges,
                            get_next_id,
                        )?;
                    }
                    edges.push(CfgEdge {
                        from_id: false_branch_id,
                        to_id: merge_id.clone(),
                        kind: CfgEdgeKind::Normal,
                        label: None,
                    });
                }
            }

            if !handled_else {
                edges.push(CfgEdge {
                    from_id: curr_cond_id,
                    to_id: merge_id.clone(),
                    kind: CfgEdgeKind::FalseBranch,
                    label: Some("✗ false".to_string()),
                });
            }

            Ok(merge_id)
        }
        "for_statement" | "while_statement" => {
            let header_id = get_next_id();
            let label = if node.kind() == "for_statement" {
                "for".to_string()
            } else {
                let cond_node = node.child_by_field_name("condition");
                if let Some(c) = cond_node {
                    format!("while {}", c.utf8_text(source_bytes).unwrap_or("condition"))
                } else {
                    "while".to_string()
                }
            };

            blocks.push(CfgBlock {
                id: header_id.clone(),
                kind: CfgBlockKind::LoopHeader,
                label,
                line,
            });
            edges.push(CfgEdge {
                from_id: current_id,
                to_id: header_id.clone(),
                kind: CfgEdgeKind::Normal,
                label: None,
            });

            let loop_exit_id = get_next_id();
            blocks.push(CfgBlock {
                id: loop_exit_id.clone(),
                kind: CfgBlockKind::Block,
                label: "loop exit".to_string(),
                line: None,
            });

            if let Some(body) = node.child_by_field_name("body") {
                let mut body_id = get_next_id();
                blocks.push(CfgBlock {
                    id: body_id.clone(),
                    kind: CfgBlockKind::LoopBody,
                    label: "loop body".to_string(),
                    line: Some(body.start_position().row as u32 + 1),
                });
                edges.push(CfgEdge {
                    from_id: header_id.clone(),
                    to_id: body_id.clone(),
                    kind: CfgEdgeKind::TrueBranch,
                    label: Some("✓ true".to_string()),
                });

                body_id = traverse_statements(
                    body,
                    source_bytes,
                    body_id,
                    exit_id,
                    &loop_exit_id,
                    Some(&header_id),
                    blocks,
                    edges,
                    get_next_id,
                )?;

                edges.push(CfgEdge {
                    from_id: body_id,
                    to_id: header_id.clone(),
                    kind: CfgEdgeKind::LoopBack,
                    label: None,
                });
            }

            // Loop break / false branch
            edges.push(CfgEdge {
                from_id: header_id.clone(),
                to_id: loop_exit_id.clone(),
                kind: CfgEdgeKind::FalseBranch,
                label: Some("✗ false".to_string()),
            });

            Ok(loop_exit_id)
        }
        "try_statement" => {
            let try_id = get_next_id();
            blocks.push(CfgBlock {
                id: try_id.clone(),
                kind: CfgBlockKind::TryBlock,
                label: "try".to_string(),
                line,
            });
            edges.push(CfgEdge {
                from_id: current_id,
                to_id: try_id.clone(),
                kind: CfgEdgeKind::Normal,
                label: None,
            });

            let try_exit_id = get_next_id();
            blocks.push(CfgBlock {
                id: try_exit_id.clone(),
                kind: CfgBlockKind::Block,
                label: "try merge".to_string(),
                line: None,
            });

            let mut final_id = try_exit_id.clone();

            if let Some(body) = node.child_by_field_name("body") {
                let last_try_id = traverse_statements(
                    body,
                    source_bytes,
                    try_id.clone(),
                    exit_id,
                    loop_exit_id,
                    loop_header_id,
                    blocks,
                    edges,
                    get_next_id,
                )?;
                edges.push(CfgEdge {
                    from_id: last_try_id,
                    to_id: try_exit_id.clone(),
                    kind: CfgEdgeKind::Normal,
                    label: None,
                });
            }

            let mut walk = node.walk();
            let handlers = node.children_by_field_name("handler", &mut walk);
            for handler in handlers {
                let catch_id = get_next_id();
                let catch_label = if let Some(type_node) = handler.child_by_field_name("type") {
                    let mut l = type_node
                        .utf8_text(source_bytes)
                        .unwrap_or("Exception")
                        .to_string();
                    if let Some(name_node) = handler.child_by_field_name("name") {
                        l = format!(
                            "{} as {}",
                            l,
                            name_node.utf8_text(source_bytes).unwrap_or("")
                        );
                    }
                    l
                } else {
                    "Exception".to_string()
                };

                blocks.push(CfgBlock {
                    id: catch_id.clone(),
                    kind: CfgBlockKind::CatchBlock,
                    label: catch_label,
                    line: Some(handler.start_position().row as u32 + 1),
                });
                edges.push(CfgEdge {
                    from_id: try_id.clone(),
                    to_id: catch_id.clone(),
                    kind: CfgEdgeKind::ExceptionEdge,
                    label: None, // Or we could put "ExceptionEdge" or similar
                });

                if let Some(body) = handler.child_by_field_name("body") {
                    let last_catch_id = traverse_statements(
                        body,
                        source_bytes,
                        catch_id,
                        exit_id,
                        loop_exit_id,
                        loop_header_id,
                        blocks,
                        edges,
                        get_next_id,
                    )?;
                    edges.push(CfgEdge {
                        from_id: last_catch_id,
                        to_id: try_exit_id.clone(),
                        kind: CfgEdgeKind::Normal,
                        label: None,
                    });
                }
            }

            // Finally
            let mut cursor = node.walk();
            for child in node.children(&mut cursor) {
                if child.kind() == "finally_clause" {
                    let finally_id = get_next_id();
                    blocks.push(CfgBlock {
                        id: finally_id.clone(),
                        kind: CfgBlockKind::FinallyBlock,
                        label: "finally".to_string(),
                        line: Some(child.start_position().row as u32 + 1),
                    });
                    edges.push(CfgEdge {
                        from_id: try_exit_id.clone(),
                        to_id: finally_id.clone(),
                        kind: CfgEdgeKind::FinallyEdge,
                        label: None,
                    });

                    if let Some(body) = child.child_by_field_name("body") {
                        final_id = traverse_statements(
                            body,
                            source_bytes,
                            finally_id,
                            exit_id,
                            loop_exit_id,
                            loop_header_id,
                            blocks,
                            edges,
                            get_next_id,
                        )?;
                    } else {
                        final_id = finally_id;
                    }
                    break;
                }
            }

            Ok(final_id)
        }
        "return_statement" => {
            let label = if let Some(first) = node.child(1) {
                // roughly value
                format!("return {}", first.utf8_text(source_bytes).unwrap_or(""))
            } else {
                "return".to_string()
            };

            // It could contain await, let's walk it quickly to find await
            let mut await_id = current_id.clone();
            let mut cursor = node.walk();
            for child in node.children(&mut cursor) {
                if child.kind() == "await" {
                    let a_id = get_next_id();
                    blocks.push(CfgBlock {
                        id: a_id.clone(),
                        kind: CfgBlockKind::AwaitPoint,
                        label: "await".to_string(),
                        line,
                    });
                    edges.push(CfgEdge {
                        from_id: await_id,
                        to_id: a_id.clone(),
                        kind: CfgEdgeKind::AsyncEdge,
                        label: None,
                    });
                    await_id = a_id;
                }
            }

            let ret_id = get_next_id();
            blocks.push(CfgBlock {
                id: ret_id.clone(),
                kind: CfgBlockKind::Block,
                label,
                line,
            });
            edges.push(CfgEdge {
                from_id: await_id,
                to_id: ret_id.clone(),
                kind: CfgEdgeKind::Normal,
                label: None,
            });

            edges.push(CfgEdge {
                from_id: ret_id.clone(),
                to_id: exit_id.to_string(),
                kind: CfgEdgeKind::Normal,
                label: None,
            });

            // Returns stop flow, so any subsequent statements in the block are unreachable
            Ok(ret_id)
        }
        "raise_statement" => {
            let label = if let Some(first) = node.child(1) {
                format!("raise {}", first.utf8_text(source_bytes).unwrap_or(""))
            } else {
                "raise".to_string()
            };

            let raise_id = get_next_id();
            blocks.push(CfgBlock {
                id: raise_id.clone(),
                kind: CfgBlockKind::ThrowPoint,
                label,
                line,
            });
            edges.push(CfgEdge {
                from_id: current_id,
                to_id: raise_id.clone(),
                kind: CfgEdgeKind::Normal,
                label: None,
            });

            // Throws exit the function normally (or caught by try which is complex to route correctly here without full scope management). We just point it to exit or let it end.
            edges.push(CfgEdge {
                from_id: raise_id.clone(),
                to_id: exit_id.to_string(),
                kind: CfgEdgeKind::ExceptionEdge,
                label: None,
            });
            Ok(raise_id)
        }
        "break_statement" => {
            let break_id = get_next_id();
            blocks.push(CfgBlock {
                id: break_id.clone(),
                kind: CfgBlockKind::Block,
                label: "break".to_string(),
                line,
            });
            edges.push(CfgEdge {
                from_id: current_id,
                to_id: break_id.clone(),
                kind: CfgEdgeKind::Normal,
                label: None,
            });
            edges.push(CfgEdge {
                from_id: break_id.clone(),
                to_id: loop_exit_id.to_string(),
                kind: CfgEdgeKind::Normal,
                label: None,
            });
            Ok(break_id)
        }
        "continue_statement" => {
            let cont_id = get_next_id();
            blocks.push(CfgBlock {
                id: cont_id.clone(),
                kind: CfgBlockKind::Block,
                label: "continue".to_string(),
                line,
            });
            edges.push(CfgEdge {
                from_id: current_id,
                to_id: cont_id.clone(),
                kind: CfgEdgeKind::Normal,
                label: None,
            });
            if let Some(h) = loop_header_id {
                edges.push(CfgEdge {
                    from_id: cont_id.clone(),
                    to_id: h.to_string(),
                    kind: CfgEdgeKind::LoopBack,
                    label: None,
                });
            }
            Ok(cont_id)
        }
        _ => {
            // Find await points in this statement
            let mut await_points = Vec::new();
            find_awaits(node, &mut await_points);

            let mut curr = current_id;
            for a_node in await_points {
                let a_id = get_next_id();
                let a_line = Some(a_node.start_position().row as u32 + 1);

                blocks.push(CfgBlock {
                    id: a_id.clone(),
                    kind: CfgBlockKind::AwaitPoint,
                    label: a_node
                        .utf8_text(source_bytes)
                        .unwrap_or("await")
                        .to_string(),
                    line: a_line,
                });
                edges.push(CfgEdge {
                    from_id: curr.clone(),
                    to_id: a_id.clone(),
                    kind: CfgEdgeKind::AsyncEdge,
                    label: None,
                });
                curr = a_id;
            }

            let text = node.utf8_text(source_bytes).unwrap_or("");
            let label = if text.len() > 30 {
                format!("{}...", &text[0..27])
            } else {
                text.to_string()
            };

            // Skip pure comments or uninteresting nodes from being blocks if desired
            if node.kind() == "comment" || text.trim().is_empty() {
                return Ok(curr);
            }

            let block_id = get_next_id();
            blocks.push(CfgBlock {
                id: block_id.clone(),
                kind: CfgBlockKind::Block,
                label: label.replace("\n", " "),
                line,
            });
            edges.push(CfgEdge {
                from_id: curr,
                to_id: block_id.clone(),
                kind: CfgEdgeKind::Normal,
                label: None,
            });
            Ok(block_id)
        }
    }
}

fn find_awaits<'a>(node: TsNode<'a>, awaits: &mut Vec<TsNode<'a>>) {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.kind() == "await" {
            awaits.push(child);
        } else {
            find_awaits(child, awaits);
        }
    }
}
