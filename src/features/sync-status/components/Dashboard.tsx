// Dashboard component (Story 3.1 AC#1)
// Main dashboard view showing all profiles with their status
import React, { useEffect } from 'react';
import { Text, Spinner, MessageBar, MessageBarBody } from '@fluentui/react-components';
import { SyncProfileCard } from './SyncProfileCard';
import { useSyncStatusStore } from '../store/syncStatusStore';
import { Add24Regular } from '@fluentui/react-icons';

export const Dashboard: React.FC = () => {
  const { 
    profiles, 
    isLoading, 
    error, 
    isInitialized,
    initialize 
  } = useSyncStatusStore();

  useEffect(() => {
    initialize();
    
    return () => {
      // Cleanup on unmount
    };
  }, []);

  // Loading state (NFR1: Dashboard loads < 3 seconds)
  if (isLoading && !isInitialized) {
    return (
      <div style={{ 
        display: 'flex', 
        flexDirection: 'column', 
        alignItems: 'center', 
        justifyContent: 'center',
        height: '100%',
        gap: '16px'
      }}>
        <Spinner size="large" label="Đang tải dashboard..." />
        <Text size={200} style={{ color: '#616161' }}>
          Vui lòng chờ trong giây lát
        </Text>
      </div>
    );
  }

  // Error state
  if (error) {
    return (
      <MessageBar intent="error">
        <MessageBarBody>{error}</MessageBarBody>
      </MessageBar>
    );
  }

  // Empty state - no profiles yet
  if (profiles.length === 0) {
    return (
      <div style={{
        display: 'flex',
        flexDirection: 'column',
        alignItems: 'center',
        justifyContent: 'center',
        height: '100%',
        gap: '16px',
        padding: '32px'
      }}>
        <Text size={400} weight="semibold">
          Chào mừng bạn đến với SyncFolder
        </Text>
        <Text size={200} style={{ color: '#616161', textAlign: 'center' }}>
          Bạn chưa có hồ sơ đồng bộ nào.
          <br />
          Tạo hồ sơ đầu tiên để bắt đầu đồng bộ thư mục.
        </Text>
        <button 
          className="fluent-button"
          style={{
            display: 'flex',
            alignItems: 'center',
            gap: '8px',
            padding: '8px 16px',
            backgroundColor: '#0078D4',
            color: 'white',
            border: 'none',
            borderRadius: '4px',
            cursor: 'pointer',
            fontSize: '14px'
          }}
        >
          <Add24Regular />
          Tạo hồ sơ đồng bộ
        </button>
      </div>
    );
  }

  // Dashboard content
  return (
    <div style={{ padding: '24px', maxWidth: '800px', margin: '0 auto' }}>
      <div style={{ marginBottom: '24px' }}>
        <Text size={500} weight="semibold">Dashboard</Text>
        <Text size={200} style={{ display: 'block', color: '#616161', marginTop: '4px' }}>
          Theo dõi trạng thái đồng bộ của các hồ sơ
        </Text>
      </div>

      {/* Profile cards */}
      <div style={{ display: 'flex', flexDirection: 'column', gap: '12px' }}>
        {profiles.map((profile) => (
          <SyncProfileCard 
            key={profile.id} 
            profile={profile}
          />
        ))}
      </div>

      {/* Add new profile button */}
      <div style={{ marginTop: '24px', textAlign: 'center' }}>
        <button 
          className="fluent-button-secondary"
          style={{
            display: 'inline-flex',
            alignItems: 'center',
            gap: '8px',
            padding: '8px 16px',
            backgroundColor: 'transparent',
            color: '#0078D4',
            border: '1px solid #0078D4',
            borderRadius: '4px',
            cursor: 'pointer',
            fontSize: '14px'
          }}
        >
          <Add24Regular />
          Thêm hồ sơ mới
        </button>
      </div>
    </div>
  );
};

export default Dashboard;
