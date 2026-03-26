import type { CSSProperties } from 'react';

interface HeaderProps {
  title: string;
}

export function Header({ title }: HeaderProps) {
  return (
    <header style={styles.header} role="banner">
      <h1 style={styles.title}>{title}</h1>
    </header>
  );
}

const styles: Record<string, CSSProperties> = {
  header: {
    display: 'flex',
    alignItems: 'center',
    padding: '16px 24px',
    backgroundColor: '#ffffff',
    borderBottom: '1px solid #e0e0e0',
  },
  title: {
    margin: 0,
    fontSize: '24px',
    fontWeight: 600,
    color: '#1f1f1f',
    fontFamily: '"Segoe UI Variable", "Segoe UI", sans-serif',
  },
};

export default Header;
