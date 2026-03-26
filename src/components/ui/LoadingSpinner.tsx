import type { CSSProperties } from 'react';
import { Spinner } from '@fluentui/react-components';

interface LoadingSpinnerProps {
  label?: string;
  size?: 'small' | 'medium' | 'large';
}

export function LoadingSpinner({ 
  label = 'Đang tải...', 
  size = 'medium'
}: LoadingSpinnerProps) {
  const spinnerSize = getSpinnerSize(size);

  return (
    <div 
      style={styles.container}
      role="status"
      aria-live="polite"
      aria-label={label}
    >
      <Spinner 
        size={spinnerSize} 
        label={label}
      />
    </div>
  );
}

function getSpinnerSize(size: 'small' | 'medium' | 'large'): 'tiny' | 'small' | 'medium' | 'large' | 'extra-large' {
  if (size === 'small') return 'small';
  if (size === 'large') return 'large';
  return 'medium';
}

const styles: Record<string, CSSProperties> = {
  container: {
    display: 'flex',
    alignItems: 'center',
    justifyContent: 'center',
    padding: '24px',
    minHeight: '100px',
  },
};

export default LoadingSpinner;
