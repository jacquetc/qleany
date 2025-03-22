import { listen } from '@tauri-apps/api/event';
// import { warn, debug, trace, info, error } from '@tauri-apps/plugin-log';
import { info } from '@tauri-apps/plugin-log';


listen('direct_access_entity_created', (event) => {
  const payload = event.payload as { ids: string[] };
  info(`Entity created event received: ${payload.ids}`);
  // Handle the event, e.g., update the UI
});

listen('direct_access_entity_updated', (event) => {
  console.log('Entity updated event received:', event);
  // Handle the event, e.g., update the UI
});

listen('direct_access_entity_removed', (event) => {
  console.log('Entity removed event received:', event);
  // Handle the event, e.g., update the UI
});

const EntityList = () => {
    return (
      <div className="">
        <h2>Entity List</h2>
        <ul>
          <li>Entity 1</li>
          <li>Entity 2</li>
          <li>Entity 3</li>
        </ul>
      </div>
    );
};
  
export default EntityList;