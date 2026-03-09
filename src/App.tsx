import { useEffect, useCallback, useState, useRef } from "react";
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
import { usePermission } from "./hooks/usePermission";
import { PermissionCard } from "./components/PermissionCard";
import type { PermissionRequest } from "./types";

function App() {
  const { requests, resolve, error } = usePermission();
  const [showExplain, setShowExplain] = useState(false);
  const [isAnimatingOut, setIsAnimatingOut] = useState(false);
  const prevRequestCount = useRef(0);
  const lastRequestRef = useRef<PermissionRequest | null>(null);

  const current = requests[0];

  // Track last non-null request for exit animation content
  useEffect(() => {
    if (current) {
      lastRequestRef.current = current;
    }
  }, [current]);

  // Handle exit animation when requests become empty
  useEffect(() => {
    const hadRequests = prevRequestCount.current > 0;
    const hasRequests = requests.length > 0;

    if (hadRequests && !hasRequests) {
      setIsAnimatingOut(true);
      const timer = setTimeout(async () => {
        setIsAnimatingOut(false);
        try {
          await getCurrentWebviewWindow().hide();
        } catch {
          // Window may already be hidden
        }
      }, 150);
      prevRequestCount.current = requests.length;
      return () => clearTimeout(timer);
    }

    prevRequestCount.current = requests.length;
  }, [requests.length]);

  const handleDeny = useCallback(() => {
    if (current) resolve(current.id, "block");
  }, [current, resolve]);

  const handleAllowOnce = useCallback(() => {
    if (current) resolve(current.id, "allow");
  }, [current, resolve]);

  const handleAllowAll = useCallback(() => {
    if (current) resolve(current.id, "allow_all", current.tool_name);
  }, [current, resolve]);

  const handleBypass = useCallback(() => {
    if (current) resolve(current.id, "bypass");
  }, [current, resolve]);

  const handleExplain = useCallback(
    (reason: string) => {
      if (current) resolve(current.id, "block", undefined, reason);
      setShowExplain(false);
    },
    [current, resolve],
  );

  const toggleExplain = useCallback(() => {
    setShowExplain((prev: boolean) => !prev);
  }, []);

  // Keyboard shortcuts
  useEffect(() => {
    const handler = (e: KeyboardEvent) => {
      if (
        e.target instanceof HTMLTextAreaElement ||
        e.target instanceof HTMLInputElement
      ) {
        return;
      }
      if (!current) return;

      if (e.key === "y") {
        e.preventDefault();
        handleAllowOnce();
      } else if (e.key === "n" || e.key === "Escape") {
        e.preventDefault();
        handleDeny();
      } else if (e.key === "a") {
        e.preventDefault();
        handleAllowAll();
      } else if (e.key === "e") {
        e.preventDefault();
        toggleExplain();
      }
    };

    window.addEventListener("keydown", handler);
    return () => window.removeEventListener("keydown", handler);
  }, [current, handleAllowOnce, handleDeny, handleAllowAll, toggleExplain]);

  const displayRequest =
    current || (isAnimatingOut ? lastRequestRef.current : null);

  if (!displayRequest) {
    return null;
  }

  const animationClass = isAnimatingOut ? "popup-exit" : "popup-enter";

  return (
    <div
      className={`bg-[#111] rounded-xl border border-[#333] shadow-2xl ${animationClass}`}
    >
      {error && (
        <div className="px-4 pt-3">
          <p className="text-xs text-red-400 bg-red-900/20 rounded px-2 py-1">
            {error}
          </p>
        </div>
      )}
      <PermissionCard
        request={displayRequest}
        pendingCount={requests.length}
        showExplain={showExplain}
        onToggleExplain={toggleExplain}
        onDeny={handleDeny}
        onAllowOnce={handleAllowOnce}
        onAllowAll={handleAllowAll}
        onBypass={handleBypass}
        onExplain={handleExplain}
      />
    </div>
  );
}

export default App;
