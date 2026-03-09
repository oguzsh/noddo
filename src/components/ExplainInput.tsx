import { useState, useRef, useEffect } from "react";

interface ExplainInputProps {
  onSend: (reason: string) => void;
  onCancel: () => void;
}

export function ExplainInput({ onSend, onCancel }: ExplainInputProps) {
  const [text, setText] = useState("");
  const textareaRef = useRef<HTMLTextAreaElement>(null);

  useEffect(() => {
    textareaRef.current?.focus();
  }, []);

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === "Enter" && e.metaKey && text.trim()) {
      onSend(text.trim());
    }
    if (e.key === "Escape") {
      onCancel();
    }
  };

  return (
    <div className="space-y-2">
      <textarea
        ref={textareaRef}
        value={text}
        onChange={(e) => setText(e.target.value)}
        onKeyDown={handleKeyDown}
        placeholder="Tell Claude what to do instead..."
        className="w-full bg-gray-900/60 border border-gray-600 rounded-lg p-2.5 text-sm text-gray-200 placeholder-gray-500 resize-none focus:outline-none focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500"
        rows={3}
      />
      <div className="flex items-center justify-between">
        <span className="text-xs text-gray-500">Cmd+Enter to send</span>
        <div className="flex gap-2">
          <button
            onClick={onCancel}
            className="px-3 py-1.5 text-xs rounded-md bg-gray-700 text-gray-300 hover:bg-gray-600 transition-colors"
          >
            Cancel
          </button>
          <button
            onClick={() => text.trim() && onSend(text.trim())}
            disabled={!text.trim()}
            className="px-3 py-1.5 text-xs rounded-md bg-blue-600 text-white hover:bg-blue-500 transition-colors disabled:opacity-40 disabled:cursor-not-allowed"
          >
            Send
          </button>
        </div>
      </div>
    </div>
  );
}
