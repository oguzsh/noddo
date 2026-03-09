import { useEffect, useState, useCallback } from "react";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";
import type { PermissionRequest, DecisionAction } from "../types";

export function usePermission() {
  const [requests, setRequests] = useState<PermissionRequest[]>([]);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const unlisten = listen<PermissionRequest>(
      "new-permission-request",
      (event) => {
        setRequests((prev) => [...prev, event.payload]);
        setError(null);
      }
    );

    invoke<PermissionRequest[]>("get_pending_requests")
      .then((pending) => {
        if (pending.length > 0) {
          setRequests(pending);
        }
      })
      .catch((e) => {
        setError(`Failed to load pending requests: ${e}`);
      });

    return () => {
      unlisten.then((fn) => fn());
    };
  }, []);

  const resolve = useCallback(
    async (
      id: string,
      action: DecisionAction,
      toolName?: string,
      reason?: string
    ) => {
      try {
        await invoke("resolve_permission", {
          id,
          action,
          reason,
          toolName,
        });
        setRequests((prev) => prev.filter((r) => r.id !== id));
        setError(null);
      } catch (e) {
        setError(`Failed to resolve: ${e}`);
      }
    },
    []
  );

  const dismiss = useCallback(async (id: string) => {
    try {
      await invoke("dismiss_request", { id });
      setRequests((prev) => prev.filter((r) => r.id !== id));
      setError(null);
    } catch (e) {
      setError(`Failed to dismiss: ${e}`);
    }
  }, []);

  return { requests, resolve, dismiss, error };
}
