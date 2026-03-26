import { AppProviders } from './app/providers';
import { AppShell } from './app/shell';
import './styles/globals.css';

function App() {
  return (
    <AppProviders>
      <AppShell>
        <main>
          <h2>Welcome to SyncFolder</h2>
          <p>Select a navigation item to get started.</p>
        </main>
      </AppShell>
    </AppProviders>
  );
}

export default App;
