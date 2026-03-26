import type { CSSProperties } from 'react';

export const sidebarStyles = {
  container: {
    display: 'flex',
    height: '100vh',
    width: '100%',
    fontFamily: '"Segoe UI Variable", "Segoe UI", sans-serif',
  } as CSSProperties,

  navigation: {
    width: '240px',
    backgroundColor: '#f3f3f3',
    borderRight: '1px solid #e0e0e0',
    display: 'flex',
    flexDirection: 'column',
  } as CSSProperties,

  content: {
    flex: 1,
    padding: '24px',
    overflow: 'auto',
    backgroundColor: '#ffffff',
  } as CSSProperties,

  navItem: {
    display: 'flex',
    alignItems: 'center',
    gap: '12px',
    width: '100%',
    padding: '12px 16px',
    border: 'none',
    backgroundColor: 'transparent',
    cursor: 'pointer',
    fontSize: '14px',
    color: '#1f1f1f',
    textAlign: 'left',
    outline: 'none',
    transition: 'background-color 0.15s ease',
  } as CSSProperties,

  navItemSelected: {
    backgroundColor: '#e5e5e5',
    borderLeft: '3px solid #0078d4',
    paddingLeft: '13px',
  } as CSSProperties,

  navIcon: {
    display: 'flex',
    alignItems: 'center',
    justifyContent: 'center',
    width: '20px',
    height: '20px',
    fontSize: '20px',
  } as CSSProperties,

  navLabel: {
    fontWeight: '400',
  } as CSSProperties,
};

export const focusStyles = {
  focusVisible: {
    outline: '2px solid #0078d4',
    outlineOffset: '-2px',
  },
};
