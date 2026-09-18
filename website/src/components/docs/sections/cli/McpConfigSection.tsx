import React from "react";
import { AslCodeBlock } from "@/components/docs/ui/AslCodeBlock";

export const McpConfigSection: React.FC = () => {
  const mcpConfig = `{
  "mcpServers": {
    "asl": {
      "command": "asl",
      "args": ["serve", "--transport", "stdio", "--dir", "/path/to/my-skills"]
    }
  }
}`;

  return (
    <div className="space-y-4 pt-4">
      <h2 className="text-xl font-bold text-white tracking-tight">
        Model Context Protocol (MCP) Configuration
      </h2>
      <p>
        You can expose your ASL skills directly to Claude Desktop, Cursor, or Google Antigravity by registering <code className="text-zinc-200 font-mono">asl serve</code> in your MCP client configuration:
      </p>

      <AslCodeBlock lang="asl" code={mcpConfig} />
    </div>
  );
};
