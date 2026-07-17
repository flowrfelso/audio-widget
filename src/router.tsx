import { Router, Route } from "@solidjs/router";

import WidgetView from "@/views/widget-view";
import SettingsView from "@/views/setting-view";

export default function AppRouter() {
  return (
    <Router>
      <Route path="/" component={WidgetView} />

      <Route path="/settings" component={SettingsView} />
    </Router>
  );
}
