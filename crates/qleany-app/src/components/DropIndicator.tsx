import React from "react";

const DropIndicator: React.FC<{ edge: "top" | "bottom"; gap: string | number }> = ({edge, gap}) => {
    const edgeClassMap = {
        top: "edge-top",
        bottom: "edge-bottom",
    };

    const edgeClass = edgeClassMap[edge as "top" | "bottom"];
    const style: React.CSSProperties = {
        "--gap": gap as string | number,
    };

    return <div className={`drop-indicator ${edgeClass}`} style={style}></div>;
};

export default DropIndicator;