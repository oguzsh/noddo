import type { PermissionRequest } from "../types";
import { ToolDetail } from "./ToolDetail";
import { ActionButtons } from "./ActionButtons";
import { ExplainInput } from "./ExplainInput";

interface PermissionCardProps {
  request: PermissionRequest;
  pendingCount: number;
  showExplain: boolean;
  onToggleExplain: () => void;
  onDeny: () => void;
  onAllowOnce: () => void;
  onAllowAll: () => void;
  onBypass: () => void;
  onExplain: (reason: string) => void;
}

export function PermissionCard({
  request,
  pendingCount,
  showExplain,
  onToggleExplain,
  onDeny,
  onAllowOnce,
  onAllowAll,
  onBypass,
  onExplain,
}: PermissionCardProps) {
  return (
    <div className="flex flex-col gap-3 p-4">
      <ToolDetail
        toolName={request.tool_name}
        toolInput={request.tool_input as Record<string, unknown>}
      />

      <ActionButtons
        onDeny={showExplain ? onToggleExplain : onDeny}
        onAllowOnce={onAllowOnce}
        onAllowAll={onAllowAll}
        onBypass={onBypass}
      />

      {showExplain && (
        <ExplainInput onSend={onExplain} onCancel={onToggleExplain} />
      )}

      {/* Footer with pending count and explain toggle */}
      <div className="flex items-center justify-between">
        {pendingCount > 1 ? (
          <span className="text-xs text-gray-500">
            Show all {pendingCount} sessions
          </span>
        ) : (
          <span />
        )}
        <button
          onClick={onToggleExplain}
          className="text-xs text-gray-500 hover:text-gray-300 transition-colors"
        >
          {showExplain ? "Hide feedback" : "Deny with feedback..."}
        </button>
      </div>
    </div>
  );
}
