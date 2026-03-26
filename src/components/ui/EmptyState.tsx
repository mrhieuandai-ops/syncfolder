import type { CSSProperties } from 'react';
import { Button } from '@fluentui/react-components';
import { Add24Regular } from '@fluentui/react-icons';
import { sidebarStyles } from '../../app/shell/sidebarStyles';

interface EmptyStateProps {
  title: string;
  description: string;
  actionLabel?: string;
  onAction?: () => void;
}

export function EmptyState({ title, description, actionLabel, onAction }: EmptyStateProps) {
  return (
    <div 
      style={styles.container}
      role="status"
      aria-live="polite"
    >
      <div style={styles.iconContainer} aria-hidden="true">
        <EmptyIcon />
      </div>
      <h3 style={styles.title}>{title}</h3>
      <p style={styles.description}>{description}</p>
      {actionLabel && onAction && (
        <Button
          appearance="primary"
          size="large"
          icon={<Add24Regular />}
          onClick={onAction}
          style={styles.button}
          aria-label={actionLabel}
        >
          {actionLabel}
        </Button>
      )}
    </div>
  );
}

function EmptyIcon() {
  return (
    <svg
      width="64"
      height="64"
      viewBox="0 0 64 64"
      fill="none"
      xmlns="http://www.w3.org/2000/svg"
      aria-hidden="true"
    >
      <rect x="8" y="12" width="48" height="40" rx="4" stroke="#0078d4" strokeWidth="2" fill="none" />
      <path d="M8 20h48" stroke="#0078d4" strokeWidth="2" />
      <circle cx="16" cy="16" r="2" fill="#0078d4" />
      <circle cx="24" cy="16" r="2" fill="#0078d4" />
      <rect x="16" y="28" width="32" height="4" rx="2" fill="#e0e0e0" />
      <rect x="16" y="36" width="24" height="4" rx="2" fill="#e0e0e0" />
    </svg>
  );
}

const styles: Record<string, CSSProperties> = {
  container: {
    display: 'flex',
    flexDirection: 'column',
    alignItems: 'center',
    justifyContent: 'center',
    padding: '48px 24px',
    textAlign: 'center',
    minHeight: '300px',
  },
  iconContainer: {
    marginBottom: '24px',
  },
  title: {
    marginBottom: '8px',
    fontSize: '20px',
    fontWeight: 600,
    color: '#1f1f1f',
  },
  description: {
    marginBottom: '24px',
    fontSize: '14px',
    color: '#616161',
    maxWidth: '400px',
  },
  button: {
    minWidth: '180px',
  },
};

export default EmptyState;
