import { useQueries, useQueryClient } from '@tanstack/vue-query'
import { computed, type Ref } from 'vue'
import { apiConnector } from '@/utils/ApiConnector'

/*
  The queries are passed as computed ref so they react to changing imageIds.
  The cache is the in-memory vue-query cache only.
*/

/**
 * Composable for managing UV image cache using TanStack Query.
 * Uses useQueries for per-image caching and provides helper functions for cache priming.
 */
export function useUvImages(imageIds: Ref<(string | number)[]>) {
  const queryClient = useQueryClient()

  // Create individual queries for each image
  const imageQueries = useQueries({
    queries: computed(() =>
      imageIds.value.map((imageId) => ({
        queryKey: ['uvImage', imageId],
        queryFn: () => apiConnector.getMediaContentAsBlobURL(Number(imageId)),
        enabled: !!imageId,
        staleTime: 10 * 60 * 1000, // 10 minutes - images don't change often
        gcTime: 30 * 60 * 1000, // 30 minutes - keep in cache longer
        retry: 2, // Retry failed image loads
      }))
    ),
  })

  // Transform results into a more usable format
  const imagesMap = computed(() => {
    const map = new Map<string | number, any>()
    imageQueries.value.forEach((query, index) => {
      const imageId = imageIds.value[index]
      if (imageId && query.data) {
        map.set(imageId, {
          id: imageId,
          data: query.data,
          isLoading: query.isLoading,
          error: query.error,
        })
      }
    })
    return map
  })

  // Helper to get a specific image
  const getImage = (imageId: string | number) => {
    return imagesMap.value.get(imageId)
  }

  // Helper to check if any images are loading
  const isLoading = computed(() => 
    imageQueries.value.some((query) => query.isLoading)
  )

  // Helper to check if any images have errors
  const hasErrors = computed(() => 
    imageQueries.value.some((query) => query.error)
  )

  // Helper to get all successfully loaded images
  const loadedImages = computed(() => 
    Array.from(imagesMap.value.values()).filter(img => img.data && !img.error)
  )

  // Prime cache with image data (useful when you already have image data from another source)
  const primeImageCache = (imageId: string | number, imageData: any) => {
    queryClient.setQueryData(['uvImage', imageId], imageData)
  }

  // Prime multiple images at once
  const primeImagesCache = (imageDataMap: Map<string | number, any>) => {
    imageDataMap.forEach((data, imageId) => {
      primeImageCache(imageId, data)
    })
  }

  // Prefetch an image (useful for lazy loading scenarios)
  const prefetchImage = async (imageId: string | number) => {
    if (!imageId) return
    
    await queryClient.prefetchQuery({
      queryKey: ['uvImage', imageId],
      queryFn: () => apiConnector.getMediaContentAsBlobURL(Number(imageId)),
      staleTime: 10 * 60 * 1000,
    })
  }

  // Invalidate and refetch all images
  const refetchImages = () => {
    return Promise.all(
      imageQueries.value.map((query) => query.refetch())
    )
  }

  return {
    // Query results
    imageQueries,
    imagesMap,
    
    // Computed helpers
    isLoading,
    hasErrors,
    loadedImages,
    
    // Utility functions
    getImage,
    primeImageCache,
    primeImagesCache,
    prefetchImage,
    refetchImages,
  }
}

// Export type for use in components
export type UvImagesComposable = ReturnType<typeof useUvImages>
