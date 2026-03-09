export interface PermissionRequest {
  id: string;
  tool_name: string;
  tool_input: Record<string, unknown>;
  received_at: string;
}

export type DecisionAction =
  | "allow"
  | "allow_all"
  | "deny"
  | "block"
  | "bypass";
