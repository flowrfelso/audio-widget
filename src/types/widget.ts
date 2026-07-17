export interface MenuItem {
  id: string;
  icon: any;
  angle: number;
  disabled?: boolean;
  isDynamic?: boolean;
  action: () => Promise<void> | void | Promise<unknown>;
}
