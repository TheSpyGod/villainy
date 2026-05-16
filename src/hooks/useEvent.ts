/*
  This method is the bridge between the calls of listen,
  each time an event is given over to the function it:
  a) Listens for the event
  b) After successful or unsuccessful try, free's up the listen subscription
  
*/

import { useEffect } from 'react';
import { listen, EventCallback } from '@tauri-apps/api/event';

export useEvent<T>(
  event: string,
  handler: EventCallback<T>,
  deps: React.DependencyList = []
) {

  useEffect( => {
    let unlisten: (() => void) | undefined;

    listen<T>(event, handler).then(fn => {
      unlisten = fn;
    });

    return () => { unlisten?.{}; };
  }, deps);

}
