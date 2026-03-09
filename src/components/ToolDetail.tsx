interface ToolDetailProps {
  toolName: string;
  toolInput: Record<string, unknown>;
}

export function ToolDetail({ toolName, toolInput }: ToolDetailProps) {
  return (
    <div className="space-y-2.5">
      {/* Tool type header — amber warning style */}
      <div className="flex items-center gap-1.5">
        <span className="text-amber-400 text-base">&#x26A0;</span>
        <span className="text-amber-400 font-semibold text-sm">{toolName}</span>
      </div>

      {/* Code preview card */}
      <div className="bg-[#1a1a1a] rounded-lg border border-[#333] overflow-hidden">
        {renderToolContent(toolName, toolInput)}
      </div>
    </div>
  );
}

function renderToolContent(
  toolName: string,
  toolInput: Record<string, unknown>
): React.ReactNode {
  switch (toolName) {
    case "Bash":
      return (
        <div className="p-3">
          <pre className="text-gray-200 text-[13px] font-mono whitespace-pre-wrap break-all leading-relaxed">
            <span className="text-gray-500">$</span>{" "}
            {String(toolInput.command ?? "")}
          </pre>
        </div>
      );

    case "Write":
      return (
        <>
          <FileHeader
            path={String(toolInput.file_path ?? "")}
            badge="new file"
          />
          {"content" in toolInput ? (
            <LineNumberedCode
              content={truncateContent(String(toolInput.content), 20)}
            />
          ) : null}
        </>
      );

    case "Edit":
      return (
        <>
          <FileHeader path={String(toolInput.file_path ?? "")} badge="edit" />
          {"old_string" in toolInput ? (
            <div className="border-t border-[#333]">
              <LineNumberedCode
                content={truncateContent(String(toolInput.old_string), 8)}
                lineClass="bg-red-950/40 text-red-300 line-through"
              />
            </div>
          ) : null}
          {"new_string" in toolInput ? (
            <div className="border-t border-[#333]">
              <LineNumberedCode
                content={truncateContent(String(toolInput.new_string), 8)}
                lineClass="bg-green-950/40 text-green-300"
              />
            </div>
          ) : null}
        </>
      );

    default:
      return (
        <div className="p-3">
          <pre className="text-gray-300 text-xs font-mono whitespace-pre-wrap leading-relaxed">
            {JSON.stringify(toolInput, null, 2)}
          </pre>
        </div>
      );
  }
}

function FileHeader({ path, badge }: { path: string; badge: string }) {
  const fileName = path.split("/").pop() ?? path;
  return (
    <div className="flex items-center gap-2 px-3 py-2">
      <span className="text-gray-200 text-[13px] font-mono">{fileName}</span>
      <span className="text-[10px] font-medium px-1.5 py-0.5 rounded bg-green-800/60 text-green-300">
        {badge}
      </span>
    </div>
  );
}

function LineNumberedCode({
  content,
  lineClass,
}: {
  content: string;
  lineClass?: string;
}) {
  const lines = content.split("\n");
  // Ensure at least 2 lines are shown (like the reference)
  while (lines.length < 2) lines.push("");

  return (
    <div className="border-t border-[#333] overflow-auto max-h-48">
      {lines.map((line, i) => (
        <div
          key={i}
          className={`flex text-[13px] font-mono leading-6 ${lineClass ?? ""}`}
        >
          <span className="w-10 shrink-0 text-right pr-3 text-gray-600 select-none">
            {i + 1}
          </span>
          <span className="text-gray-200 whitespace-pre pr-3">
            {line || " "}
          </span>
        </div>
      ))}
    </div>
  );
}

function truncateContent(content: string, maxLines: number): string {
  const lines = content.split("\n");
  if (lines.length <= maxLines) return content;
  const half = Math.floor(maxLines / 2);
  const hidden = lines.length - maxLines;
  return [
    ...lines.slice(0, half),
    `  ... ${hidden} lines hidden ...`,
    ...lines.slice(-half),
  ].join("\n");
}
