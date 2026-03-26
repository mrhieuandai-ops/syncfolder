import type { CSSProperties } from 'react';
import { MessageBar, MessageBarBody, MessageBarTitle } from '@fluentui/react-components';
import { 
  CheckmarkCircle24Regular,
  Warning24Regular,
  Dismiss24Regular,
  Info24Regular
} from '@fluentui/react-icons';

export type FeedbackType = 'success' | 'warning' | 'error' | 'info';

interface FeedbackMessageProps {
  type: FeedbackType;
  title: string;
  description?: string;
  dismissible?: boolean;
  onDismiss?: () => void;
}

const iconMap = {
  success: <CheckmarkCircle24Regular />,
  warning: <Warning24Regular />,
  error: <Dismiss24Regular />,
  info: <Info24Regular />,
};

const ariaLiveMap = {
  success: 'polite' as const,
  warning: 'polite' as const,
  error: 'assertive' as const,
  info: 'polite' as const,
};

export function FeedbackMessage({ 
  type, 
  title, 
  description, 
  dismissible = false,
  onDismiss 
}: FeedbackMessageProps) {
  return (
    <MessageBar
      style={styles.messageBar}
      intent={type}
      role={type === 'error' ? 'alert' : 'status'}
      aria-live={ariaLiveMap[type]}
    >
      <div style={styles.content}>
        <span style={styles.icon} aria-hidden="true">
          {iconMap[type]}
        </span>
        <MessageBarBody>
          <MessageBarTitle>{title}</MessageBarTitle>
          {description && <p style={styles.description}>{description}</p>}
        </MessageBarBody>
      </div>
      {dismissible && onDismiss && (
        <button 
          onClick={onDismiss}
          style={styles.dismissButton}
          aria-label="Đóng thông báo"
        >
          ✕
        </button>
      )}
    </MessageBar>
  );
}

const styles: Record<string, CSSProperties> = {
  messageBar: {
    marginBottom: '16px',
  },
  content: {
    display: 'flex',
    alignItems: 'flex-start',
    gap: '12px',
  },
  icon: {
    fontSize: '20px',
    display: 'flex',
    alignItems: 'center',
    marginTop: '2px',
  },
  description: {
    marginTop: '4px',
    fontSize: '13px',
    color: '#616161',
  },
  dismissButton: {
    background: 'none',
    border: 'none',
    cursor: 'pointer',
    fontSize: '12px',
    padding: '4px 8px',
    marginLeft: 'auto',
    color: '#616161',
  },
};

export default FeedbackMessage;
