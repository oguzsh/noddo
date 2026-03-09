interface ActionButtonsProps {
  onDeny: () => void;
  onAllowOnce: () => void;
  onAllowAll: () => void;
  onBypass: () => void;
}

export function ActionButtons({
  onDeny,
  onAllowOnce,
  onAllowAll,
  onBypass,
}: ActionButtonsProps) {
  return (
    <div className="flex gap-2">
      <button
        onClick={onDeny}
        className="flex-1 px-3 py-2.5 rounded-lg bg-[#2a2a2a] border border-[#444] text-gray-200 text-sm font-medium hover:bg-[#333] active:bg-[#222] transition-colors"
      >
        Deny
      </button>
      <button
        onClick={onAllowOnce}
        className="flex-1 px-3 py-2.5 rounded-lg bg-[#2a2a2a] border border-[#555] text-gray-100 text-sm font-medium hover:bg-[#333] active:bg-[#222] transition-colors"
      >
        Allow Once
      </button>
      <button
        onClick={onAllowAll}
        className="flex-1 px-3 py-2.5 rounded-lg bg-amber-600 text-white text-sm font-medium hover:bg-amber-500 active:bg-amber-700 transition-colors"
      >
        Allow All
      </button>
      <button
        onClick={onBypass}
        className="flex-1 px-3 py-2.5 rounded-lg bg-red-700 text-white text-sm font-medium hover:bg-red-600 active:bg-red-800 transition-colors"
      >
        Bypass
      </button>
    </div>
  );
}
