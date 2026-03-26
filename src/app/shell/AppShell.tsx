import { useState, useCallback, useRef, useEffect } from 'react';
import { navItems } from './navItems';
import { sidebarStyles, focusStyles } from './sidebarStyles';

export type NavItem = 'dashboard' | 'profiles' | 'activity-log' | 'settings';

interface AppShellProps {
  children: React.ReactNode;
}

export function AppShell({ children }: AppShellProps) {
  const [selectedItem, setSelectedItem] = useState<NavItem>('dashboard');
  const navRefs = useRef<(HTMLButtonElement | null)[]>([]);

  const handleNavItemClick = useCallback((item: NavItem) => {
    setSelectedItem(item);
  }, []);

  const handleKeyDown = useCallback((e: React.KeyboardEvent, index: number) => {
    const items = navItems;
    let newIndex = index;

    if (e.key === 'ArrowDown' || e.key === 'ArrowRight') {
      e.preventDefault();
      newIndex = (index + 1) % items.length;
    } else if (e.key === 'ArrowUp' || e.key === 'ArrowLeft') {
      e.preventDefault();
      newIndex = (index - 1 + items.length) % items.length;
    } else if (e.key === 'Enter' || e.key === ' ') {
      e.preventDefault();
      handleNavItemClick(items[index].key);
      return;
    }

    if (newIndex !== index) {
      navRefs.current[newIndex]?.focus();
      setSelectedItem(items[newIndex].key);
    }
  }, [handleNavItemClick]);

  useEffect(() => {
    const selectedIndex = navItems.findIndex(item => item.key === selectedItem);
    navRefs.current[selectedIndex]?.focus();
  }, [selectedItem]);

  return (
    <div className="app-shell" style={sidebarStyles.container}>
      <nav 
        style={sidebarStyles.navigation}
        aria-label="Điều hướng chính"
        role="menubar"
      >
        {navItems.map((item, index) => (
          <button
            key={item.key}
            ref={(el) => { navRefs.current[index] = el; }}
            role="menuitemradio"
            aria-label={item.label}
            aria-checked={selectedItem === item.key}
            data-key={item.key}
            onClick={() => handleNavItemClick(item.key)}
            style={{
              ...sidebarStyles.navItem,
              ...(selectedItem === item.key ? sidebarStyles.navItemSelected : {}),
            }}
            onKeyDown={(e) => handleKeyDown(e, index)}
            tabIndex={selectedItem === item.key ? 0 : -1}
            onFocus={(e) => {
              if (e.currentTarget === e.target) {
                Object.assign(e.currentTarget.style, focusStyles.focusVisible);
              }
            }}
          >
            <span style={sidebarStyles.navIcon} aria-hidden="true">
              {item.icon}
            </span>
            <span style={sidebarStyles.navLabel}>{item.label}</span>
          </button>
        ))}
      </nav>
      <div style={sidebarStyles.content}>
        {children}
      </div>
    </div>
  );
}

export { AppShell as default };
