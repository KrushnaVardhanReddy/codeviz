import React from 'react';
import { BaseEdge, EdgeProps, getBezierPath } from '@xyflow/react';
import { EDGE_COLORS } from '../../lib/colorMap';

const CustomEdge: React.FC<EdgeProps> = ({
  sourceX,
  sourceY,
  targetX,
  targetY,
  sourcePosition,
  targetPosition,
  style = {},
  data,
  markerEnd,
}) => {
  const [edgePath] = getBezierPath({
    sourceX,
    sourceY,
    sourcePosition,
    targetX,
    targetY,
    targetPosition,
  });

  const edgeKind = (data?.kind as string) || 'Imports';
  const colors = EDGE_COLORS[edgeKind] || EDGE_COLORS['Imports'];

  // Parse custom style description into CSS values
  let strokeDasharray = 'none';
  let strokeWidth = 1;

  if (colors.style.toLowerCase().includes('dashed')) {
    strokeDasharray = '5 5';
  } else if (colors.style.toLowerCase().includes('dotted')) {
    strokeDasharray = '2 2';
  }

  if (colors.style.toLowerCase().includes('thick')) {
    strokeWidth = 2;
  } else if (colors.style.toLowerCase().includes('thin')) {
    strokeWidth = 1; // Assuming default is 1, thin might be 0.5 or 1
  }

  return (
    <>
      <BaseEdge
        path={edgePath}
        style={{
          ...style,
          stroke: colors.color,
          strokeWidth,
          strokeDasharray,
        }}
        markerEnd={markerEnd}
      />
    </>
  );
};

export default CustomEdge;
