import React, { FC, useEffect, useRef, useState } from 'react';

interface ListItemProps {
  title: string;
  subtitle?: string;
  moveable?: boolean;
  onRemove?: () => void;
  onSelect?: () => void;
  selected?: boolean;
  onNavigate?: (direction: 'up' | 'down') => void;
}

const ListItem: FC<ListItemProps> = ({ title, subtitle, moveable, onRemove, onSelect, selected, onNavigate }) => {
  const itemRef = useRef<HTMLDivElement>(null);
  const [showConfirm, setShowConfirm] = useState(false);
  const noButtonRef = useRef<HTMLButtonElement>(null);
  const [contextMenuVisible, setContextMenuVisible] = useState(false);
  const [contextMenuPosition, setContextMenuPosition] = useState({ x: 0, y: 0 });

  useEffect(() => {
    if (showConfirm) {
      (document.getElementById('confirmModal') as HTMLDialogElement)?.showModal();
      noButtonRef.current?.focus();
    } else {
        (document.getElementById('confirmModal') as HTMLDialogElement)?.close();
    }
  }
  , [showConfirm]);

  const handleRemove = () => {
    setShowConfirm(false);
    onRemove && onRemove();
  };

  const handleContextMenu = (event: React.MouseEvent) => {
    event.preventDefault();
    setContextMenuPosition({ x: event.clientX, y: event.clientY });
    setContextMenuVisible(true);
  };

  const handleClickOutside = (event: MouseEvent) => {
    if (itemRef.current && !itemRef.current.contains(event.target as Node)) {
      setContextMenuVisible(false);
    }
  };

  useEffect(() => {
    document.addEventListener('mousedown', handleClickOutside);
    return () => {
      document.removeEventListener('mousedown', handleClickOutside);
    };
  }, []);

  return (
    <div
      ref={itemRef}
      role="button"
      tabIndex={0}
      className={`flex items-center justify-between p-4 ${selected ? 'bg-base-300' : ''}`}
      aria-pressed={selected}
      aria-label={`Select ${title}`}
      onClick={() => onSelect && onSelect() }
      onContextMenu={handleContextMenu}
      onKeyDown={(e) => {
        if (e.key === 'Enter' || e.key === ' ') {
          onSelect && onSelect();
        } else if (e.key === 'Delete') {
          setShowConfirm(true);
        } else if (e.key === 'ArrowUp') {
          onNavigate && onNavigate('up');
        } else if (e.key === 'ArrowDown') {
          onNavigate && onNavigate('down');
        }
      }}
    >
     {/* grab button */}
      {moveable && (
        <button
          tabIndex={-1}
          className="cursor-move h-full flex items-center justify-center p-2"
          aria-label="Move item"
        >
          <svg className="h-6 w-6" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
            <circle cx="12" cy="5" r="1" />
            <circle cx="12" cy="12" r="1" />
            <circle cx="12" cy="19" r="1" />
            <circle cx="5" cy="5" r="1" />
            <circle cx="5" cy="12" r="1" />
            <circle cx="5" cy="19" r="1" />
            <circle cx="19" cy="5" r="1" />
            <circle cx="19" cy="12" r="1" />
            <circle cx="19" cy="19" r="1" />
          </svg>
        </button>
      )}
      <div className="flex flex-col space-y-1 px-2">
        <span>{title}</span>
        {subtitle && <span className="text-sm italic">{subtitle}</span>}
      </div>
      <div className="dropdown">
        <div tabIndex={-1} className="btn m-1 btn-ghost btn-s">⋮</div>
        <ul tabIndex={-1} className="menu dropdown-content bg-base-100 rounded-box z-1 w-52 p-2 shadow-sm">
          <li>
            <button aria-label={`Remove ${title}`} onClick={() => setShowConfirm(true)}>Remove</button>
          </li>
        </ul>
      </div>

      {/* context menu */}
      {contextMenuVisible && (
        <ul
          className="menu bg-base-100 rounded-box z-10 p-2 shadow-sm"
          style={{ position: 'absolute', top: contextMenuPosition.y, left: contextMenuPosition.x }}
        >
          <li>
            <button aria-label={`Remove ${title}`} onClick={() => { setShowConfirm(true); setContextMenuVisible(false); }}>Remove</button>
          </li>
        </ul>
      )}

      {/* confirm dialog */}
      <dialog id="confirmModal" className="modal" 
      onKeyDown={(e) => {
        if (e.key === 'Escape'){
             setShowConfirm(false)
            }
            
        else if (e.key === 'Enter') {
            handleRemove()
            setShowConfirm(false)
        }
      }
      }
            >
        <div className="modal-box bg-base-100 p-4">
          <h3 className="font-bold text-lg">Confirm Deletion</h3>
          <p className="py-4">Are you sure you want to delete {title}?</p>
          <div className="modal-action">
            <button className="btn btn-primary" onClick={handleRemove}>Yes</button>
            <button ref={noButtonRef} className="btn btn-secondary" onClick={() => setShowConfirm(false) }>No</button>
          </div>
        </div>  
      </dialog>
    </div>
  );
};

export default ListItem;
