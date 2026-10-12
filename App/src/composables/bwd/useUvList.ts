import { type Ref, computed } from 'vue'
import { useQuery } from '@tanstack/vue-query'

import type { UntersuchungListItem } from '@/store/bwd/UntersuchungsverfahrenStore'
import { apiConnector } from '@/utils/ApiConnector'

// Bundle every parameter that affects the result into the queryKey
export function useUvList (props: {
  art: Ref<any>
  bauwerkId: Ref<string | null>
  projectId: Ref<string | null>
  filters?: {
    stelle: Ref<number | null>
    verfahren: Ref<string[]>
  }
}) {
  const queryKey = computed(() => ([
    'uv-list',
    props.art.value,
    props.bauwerkId.value,
    props.projectId.value,
    props.filters ? {
      stelle: props.filters.stelle.value,
      verfahren: props.filters.verfahren.value
    } : null
  ]))

  const query = useQuery<UntersuchungListItem[]>({
    queryKey,
    queryFn: async () => {
      // Filtering happens locally in the view, so no filter params are sent.
      const filters = props.filters ? {
        stelle: props.filters.stelle.value,
        verfahren: props.filters.verfahren.value
      } : {};
      
      const data = await apiConnector.BWD.fetchUvData(props.art.value, props.bauwerkId.value ?? '', props.projectId.value ?? '', filters);
      
      const artData = data[props.art.value];
      if (!artData || typeof artData !== 'object') {
        return [];
      }

      //get all arrays and flatten them
      const investigations = Object.entries(artData)
        .flatMap(([investigationType, investigationList]) => 
          Array.isArray(investigationList) 
            ? investigationList.map(item => ({ ...item, type: investigationType }))
            : []
        );
      
      return investigations;
    },
    refetchOnWindowFocus: true,
    refetchOnReconnect: true,  
    refetchOnMount: true,
    retry: 3,
  })

  return query
}
