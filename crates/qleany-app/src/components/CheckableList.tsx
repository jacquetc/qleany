import React, { ReactNode, useCallback, useEffect, useMemo, useState } from 'react';
import { ActionIcon, Box, Checkbox, Group, Text, TextInput } from '@mantine/core';
import cx from 'clsx';
import classes from './DndListHandle.module.css';
import { IconCheck, IconSearch, IconSquare, IconX } from '@tabler/icons-react';
import { AutoSizer, List } from 'react-virtualized';

// SearchInput sub-component
interface SearchInputProps {
  value: string;
  onChange: (value: string) => void;
  itemType: string;
}

const SearchInput = ({ value, onChange, itemType }: SearchInputProps) => {
  const handleSearchChange = useCallback((event: React.ChangeEvent<HTMLInputElement>) => {
    onChange(event.currentTarget.value);
  }, [onChange]);

  const clearSearch = useCallback(() => {
    onChange('');
  }, [onChange]);

  return (
    <Box mb={10}>
      <TextInput
        placeholder={`Search ${itemType}s...`}
        value={value}
        onChange={handleSearchChange}
        size="sm"
        icon={<IconSearch size={16}/>}
        rightSection={
          value ? (
            <ActionIcon onClick={clearSearch} size="sm" variant="transparent">
              <IconX size={16}/>
            </ActionIcon>
          ) : null
        }
        styles={{
          root: {marginBottom: '10px'}
        }}
      />
    </Box>
  );
};

// HeaderWithControls sub-component
interface HeaderWithControlsProps {
  header: ReactNode;
  allChecked: boolean;
  onToggleAll: () => void;
}

const HeaderWithControls = ({ header, allChecked, onToggleAll }: HeaderWithControlsProps) => {
  return (
    <Box style={{display: 'flex', justifyContent: 'space-between', alignItems: 'center'}}>
      <div>{header}</div>
      <ActionIcon
        onClick={onToggleAll}
        title={allChecked ? "Uncheck all" : "Check all"}
        color={allChecked ? "blue" : "gray"}
        variant="subtle"
        size="lg"
      >
        {allChecked ? <IconCheck size={20}/> : <IconSquare size={20}/>}
      </ActionIcon>
    </Box>
  );
};

// ListItem sub-component
interface ListItemProps<T> {
  item: T;
  style: React.CSSProperties;
  isSelected: boolean;
  isChecked: boolean;
  onSelect: () => void;
  onCheck: (checked: boolean) => void;
  renderContent: (item: T) => ReactNode;
}

function ListItem<T>({ 
  item, 
  style, 
  isSelected, 
  isChecked, 
  onSelect, 
  onCheck, 
  renderContent 
}: ListItemProps<T>) {
  const handleCheckboxChange = useCallback((event: React.ChangeEvent<HTMLInputElement>) => {
    event.stopPropagation();
    onCheck(event.currentTarget.checked);
  }, [onCheck]);

  const handleCheckboxClick = useCallback((event: React.MouseEvent) => {
    event.stopPropagation();
  }, []);

  // Render content with fallback for errors
  let content: ReactNode;
  try {
    content = renderContent(item);
  } catch (err) {
    console.error("Error rendering item content:", err);
    content = <Text color="red">Error rendering item</Text>;
  }

  return (
    <Group
      align="left"
      className={cx(classes.item, {
        [classes.itemSelected]: isSelected
      })}
      onClick={onSelect}
      style={{...style, margin: 0}}
    >
      <Checkbox
        checked={isChecked}
        onChange={handleCheckboxChange}
        onClick={handleCheckboxClick}
        className="ml-2"
      />
      {content}
    </Group>
  );
}

interface CheckableListProps<T> {
  items: T[];
  selectedItemId: number | null;
  checkedItemIds: number[];
  onSelectItem: (itemId: number) => void;
  onCheckItem: (itemId: number, checked: boolean) => void;
  getItemId: (item: T) => number;
  renderItemContent: (item: T) => ReactNode;
  itemType: string;
  header?: ReactNode;
  sortItems?: (a: T, b: T) => number;
  filterItem?: (item: T, query: string) => boolean;
}

function CheckableList<T>({
  items,
  selectedItemId,
  checkedItemIds,
  onSelectItem,
  onCheckItem,
  getItemId,
  renderItemContent,
  itemType,
  header,
  sortItems,
  filterItem
}: CheckableListProps<T>) {
  const [listItems, setListItems] = useState<T[]>([]);
  const [searchQuery, setSearchQuery] = useState('');
  
  // Check if all items are checked
  const allChecked = useMemo(() => {
    if (!Array.isArray(items) || items.length === 0) {
      return false;
    }
    
    return items.every(item => {
      try {
        const itemId = getItemId(item);
        return checkedItemIds.includes(itemId);
      } catch {
        return false;
      }
    });
  }, [items, checkedItemIds, getItemId]);

  // Function to handle check/uncheck all
  const handleToggleAll = useCallback(() => {
    if (!Array.isArray(items) || items.length === 0) {
      return;
    }
    
    if (allChecked) {
      // Uncheck all items
      items.forEach(item => {
        try {
          const itemId = getItemId(item);
          if (checkedItemIds.includes(itemId)) {
            onCheckItem(itemId, false);
          }
        } catch (err) {
          console.error("Error unchecking item:", err);
        }
      });
    } else {
      // Check all items
      items.forEach(item => {
        try {
          const itemId = getItemId(item);
          if (!checkedItemIds.includes(itemId)) {
            onCheckItem(itemId, true);
          }
        } catch (err) {
          console.error("Error checking item:", err);
        }
      });
    }
  }, [items, allChecked, checkedItemIds, getItemId, onCheckItem]);

  // Filter and sort items based on search query
  useEffect(() => {
    if (!Array.isArray(items)) {
      setListItems([]);
      return;
    }
    
    // Filter out any null or undefined items
    let filteredItems = items.filter(item => item != null);
    
    // Apply search filter if there's a query
    if (searchQuery.trim()) {
      filteredItems = filteredItems.filter(item => {
        // Use custom filter function if provided
        if (filterItem) {
          return filterItem(item, searchQuery.trim());
        }
        
        // Default filtering approach
        try {
          const itemAsString = JSON.stringify(item).toLowerCase();
          return itemAsString.includes(searchQuery.toLowerCase());
        } catch {
          return true; // Keep item if filtering fails
        }
      });
    }
    
    // Apply sorting if provided
    if (sortItems) {
      try {
        filteredItems = [...filteredItems].sort(sortItems);
      } catch (err) {
        console.error("Error sorting items:", err);
      }
    }
    
    setListItems(filteredItems);
  }, [items, sortItems, searchQuery, filterItem]);

  // Create a row renderer function for react-virtualized
  const rowRenderer = useCallback(({
    key,
    index,
    style
  }: {
    key: string;
    index: number;
    isScrolling: boolean;
    isVisible: boolean;
    style: React.CSSProperties;
  }) => {
    const item = listItems[index];
    if (!item) {
      return null;
    }
    
    let itemId: number;
    try {
      itemId = getItemId(item);
    } catch (err) {
      console.error("Error getting item ID:", err);
      return null;
    }
    
    const isChecked = checkedItemIds.includes(itemId);
    const isSelected = itemId === selectedItemId;
    
    return (
      <div key={key}>
        <ListItem
          item={item}
          style={style}
          isSelected={isSelected}
          isChecked={isChecked}
          onSelect={() => onSelectItem(itemId)}
          onCheck={(checked) => onCheckItem(itemId, checked)}
          renderContent={renderItemContent}
        />
      </div>
    );
  }, [listItems, checkedItemIds, selectedItemId, getItemId, renderItemContent, onSelectItem, onCheckItem]);

  // Create header with controls if header is provided
  const headerWithControls = header ? (
    <HeaderWithControls 
      header={header} 
      allChecked={allChecked} 
      onToggleAll={handleToggleAll} 
    />
  ) : null;

  // Define a constant for item height
  const ITEM_HEIGHT = 40; // Adjust based on your actual item height

  return (
    <>
      <SearchInput 
        value={searchQuery} 
        onChange={setSearchQuery} 
        itemType={itemType} 
      />
      {headerWithControls}
      <div
        style={{
          height: 'calc(100% - 80px)', // Adjust based on your header and search input height
          flexGrow: 1,
          display: 'flex',
          flexDirection: 'column'
        }}
      >
        {listItems.length > 0 ? (
          <div style={{flex: 1, minHeight: 0}}>
            <AutoSizer>
              {({width, height}) => (
                <List
                  width={width}
                  height={height}
                  rowCount={listItems.length}
                  rowHeight={ITEM_HEIGHT}
                  overscanRowCount={5} // Render a few extra items for smoother scrolling
                  rowRenderer={rowRenderer}
                />
              )}
            </AutoSizer>
          </div>
        ) : (
          <div style={{padding: '10px'}}>No items found</div>
        )}
      </div>
    </>
  );
}

export default CheckableList;
