import { HashRouter, Route, Routes } from "react-router-dom";
import AppShell from "./components/layout/AppShell";
import TodayScreen from "./features/today/TodayScreen";
import ReviewScreen from "./features/review/ReviewScreen";
import WordsListScreen from "./features/words/WordsListScreen";
import MapScreen from "./features/map/MapScreen";
import SettingsScreen from "./features/settings/SettingsScreen";

export default function AppRouter() {
  return (
    <HashRouter>
      <Routes>
        <Route element={<AppShell />}>
          <Route path="/" element={<TodayScreen />} />
          <Route path="/revisar" element={<ReviewScreen />} />
          <Route path="/palavras" element={<WordsListScreen />} />
          <Route path="/mapa" element={<MapScreen />} />
          <Route path="/configuracoes" element={<SettingsScreen />} />
        </Route>
      </Routes>
    </HashRouter>
  );
}
