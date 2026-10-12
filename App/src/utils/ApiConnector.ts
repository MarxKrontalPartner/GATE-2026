import router from '@/router/index';
import { useBauwerkStore } from '@/store/BauwerkStore';
import type {
  IBauwerk,
  IFirma,
  IPerson,
  IProjekt,
} from '@/types/apiInterfaces';
import {
  ASSETVERWALTUNG_SUBPATH,
  BAUWERKE_ENDPOINT,
  FIRMEN_ENDPOINT,
  PERSONEN_ENDPOINT,
  PROJEKTE_ENDPOINT,
} from '@/config/endpoints';
import { getMethodApiName } from '@/utils/UntersuchungsverfahrenRegistry';

// The base URL must not end with a slash, because the BWD helpers resolve
// relative URLs like new URL('api/bwd/...', API_BASE_URL) against it.
const API_BASE_URL: string =
  import.meta.env.VITE_VARS_API_BASE_URL ?? 'http://127.0.0.1:8000/api';
const DIAGNOSTIK_SUBPATH = 'bwd';

// ToDO might to put this in a proper class.
type IMapping = {
  [key: string]: string;
};
class ApiConnector {
  GET = 'GET';
  POST = 'POST';
  PATCH = 'PATCH';
  DELETE = 'DELETE';
  PUT = 'PUT';

  constructor() { }
  async getAuthHeaders(): Promise<IMapping> {
    // No authentication is used.
    return {};
  }
  async makeHeaders(additionalHeaders: IMapping = {}): Promise<IMapping> {
    // authHeaders are always needed but additional headers can be provided
    return { ...(await this.getAuthHeaders()), ...additionalHeaders };
  }
  getSelectedBauwerk(): IBauwerk | null {
    return useBauwerkStore().selectedBauwerk;
  }
  getActiveBauwerk(): IBauwerk {
    return useBauwerkStore().activeBauwerk;
  }

  handleError(error: any): any {
    console.log(error);
  }

  formatErrorPayload(errorPayload: unknown): string {
    if (errorPayload === null || errorPayload === undefined) {
      return '';
    }
    if (typeof errorPayload === 'string') {
      return errorPayload;
    }
    if (Array.isArray(errorPayload)) {
      return errorPayload
        .map((item) => this.formatErrorPayload(item))
        .filter(Boolean)
        .join(', ');
    }
    if (typeof errorPayload === 'object') {
      return Object.entries(errorPayload as Record<string, unknown>)
        .map(([key, value]) => {
          const formattedValue = this.formatErrorPayload(value);
          return formattedValue ? `${key}: ${formattedValue}` : key;
        })
        .filter(Boolean)
        .join(' | ');
    }
    return String(errorPayload);
  }

  async createHttpError(response: Response): Promise<Error> {
    const fallbackMessage = `request failed with status ${response.status}, reason: ${response.statusText}`;
    try {
      const contentType = response.headers.get('content-type') ?? '';
      if (contentType.includes('application/json')) {
        const errorPayload = await response.json();
        const formattedPayload = this.formatErrorPayload(errorPayload);
        if (formattedPayload) {
          return new Error(
            `request failed with status ${response.status}, reason: ${formattedPayload}`,
          );
        }
      }

      const errorText = (await response.text()).trim();
      if (errorText) {
        return new Error(
          `request failed with status ${response.status}, reason: ${errorText}`,
        );
      }
    } catch {
      // Fall back to the default message when the body cannot be parsed.
    }

    return new Error(fallbackMessage);
  }

  makeQueryParams(
    includeBauwerkQuery: boolean = false,
    params: IMapping = {},
  ): string {
    const queryies = [];
    // add provided query params
    for (const [key, value] of Object.entries(params)) {
      queryies.push(`${key}=${value}`);
    }
    // solves vite hmr issue where the router path is not set when reloading different files
    if (router.currentRoute.value.name === undefined) {
      router.push(window.location.pathname);
    }
    // include bauwerk_id in query if needed and when a bauwerk is selected
    if (includeBauwerkQuery) {
      if (router.currentRoute.value.params.bauwerkId) {
        queryies.push(
          `bauwerk_id=${router.currentRoute.value.params.bauwerkId}`,
        );
      }
    }
    // join queries and return
    return queryies.length > 0 ? `?${queryies.join('&')}` : '';
  }

  async getResourceList(
    resource: string,
    queryParams: IMapping = {},
  ): Promise<any[]> {
    const response = await fetch(
      `${API_BASE_URL}/${ASSETVERWALTUNG_SUBPATH}/${resource}/` +
      this.makeQueryParams(true, queryParams),
      {
        method: this.GET,
        headers: await this.makeHeaders(),
      },
    );
    // error if request failed
    if (response.status !== 200) {
      throw new Error(
        `request failed with status ${response.status}, reason: ${response.statusText}`,
      );
    }
    return await response.json();
  }
  async getXResourceList<T>(
    subpath: string,
    resource: string,
    projekt?: string,
    bauwerk?: string,
    extraParams: Record<
      string,
      string | number | boolean | null | undefined
    > = {},
  ): Promise<T[]> {
    const queryParams: IMapping = {};

    if (projekt) {
      queryParams.projekt_id = projekt;
    }
    if (bauwerk) {
      queryParams.bauwerk_id = bauwerk; //TODO: check if this is correct
    }
    // merge extra params (skip null/undefined)
    for (const [k, v] of Object.entries(extraParams)) {
      if (v !== null && v !== undefined) {
        queryParams[k] = String(v);
      }
    }

    const response = await fetch(
      `${API_BASE_URL}/${subpath}/${resource}/` +
      this.makeQueryParams(!bauwerk, queryParams),
      {
        method: this.GET,
        headers: await this.makeHeaders(),
      },
    );

    if (response.status !== 200) {
      throw new Error(
        `request failed with status ${response.status}, reason: ${response.statusText}`,
      );
    }

    return await response.json();
  }
  async getXResource<T>(
    subpath: string,
    resource: string,
    id: number | string,
  ): Promise<T> {
    const queryParams: IMapping = {};

    const response = await fetch(
      `${API_BASE_URL}/${subpath}/${resource}/${id}/` +
      this.makeQueryParams(true, queryParams),
      {
        method: this.GET,
        headers: await this.makeHeaders(),
      },
    );
    // error if request failed
    if (response.status !== 200) {
      throw new Error(
        `request failed with status ${response.status}, reason: ${response.statusText}`,
      );
    }
    return await response.json();
  }

  async getResource(resource: string, id: number | string): Promise<any> {
    const response = await fetch(
      `${API_BASE_URL}/${ASSETVERWALTUNG_SUBPATH}/${resource}/${id}/` +
      this.makeQueryParams(true),
      {
        method: this.GET,
        headers: await this.makeHeaders(),
      },
    );
    // error if request failed
    if (response.status !== 200) {
      throw new Error(
        `request failed with status ${response.status}, reason: ${response.statusText}`,
      );
    }
    return await response.json();
  }

  async getBauwerke(): Promise<IBauwerk[]> {
    const response = await fetch(
      `${API_BASE_URL}/${ASSETVERWALTUNG_SUBPATH}/${BAUWERKE_ENDPOINT}/`,
      {
        method: this.GET,
        headers: await this.makeHeaders(),
      },
    );
    // error if request failed
    if (response.status !== 200) {
      throw new Error(
        `request failed with status ${response.status}, reason: ${response.statusText}`,
      );
    }
    return await response.json();
  }

  async getProjekte(all = true): Promise<IProjekt[]> {
    const response = await fetch(
      `${API_BASE_URL}/${ASSETVERWALTUNG_SUBPATH}/${PROJEKTE_ENDPOINT}/` +
      this.makeQueryParams(!all),
      {
        method: this.GET,
        headers: await this.makeHeaders(),
      },
    );
    // error if request failed
    if (response.status !== 200) {
      throw new Error(
        `request failed with status ${response.status}, reason: ${response.statusText}`,
      );
    }
    return await response.json();
  }

  async getPersonen(all = true): Promise<IPerson[]> {
    const response = await fetch(
      `${API_BASE_URL}/${ASSETVERWALTUNG_SUBPATH}/${PERSONEN_ENDPOINT}/` +
      this.makeQueryParams(!all),
      {
        method: this.GET,
        headers: await this.makeHeaders(),
      },
    );
    // error if request failed
    if (response.status !== 200) {
      throw new Error(
        `request failed with status ${response.status}, reason: ${response.statusText}`,
      );
    }
    return await response.json();
  }

  async getFirmen(all = true): Promise<IFirma[]> {
    const response = await fetch(
      `${API_BASE_URL}/${ASSETVERWALTUNG_SUBPATH}/${FIRMEN_ENDPOINT}/` +
      this.makeQueryParams(!all),
      {
        method: this.GET,
        headers: await this.makeHeaders(),
      },
    );
    // error if request failed
    if (response.status !== 200) {
      throw new Error(
        `request failed with status ${response.status}, reason: ${response.statusText}`,
      );
    }
    return await response.json();
  }

  async postResource(resource: string, data: object): Promise<object> {
    const response = await fetch(
      `${API_BASE_URL}/${ASSETVERWALTUNG_SUBPATH}/${resource}/`,
      {
        method: this.POST,
        body: JSON.stringify(data),
        headers: await this.makeHeaders({ 'Content-Type': 'application/json' }),
      },
    );
    // error if request failed
    if (response.status !== 201) {
      throw await this.createHttpError(response);
    }
    return await response.json();
  }
  async postxResource(
    sub_path: string,
    resource: string,
    data: object,
    headers: any | undefined = undefined,
  ): Promise<any> {
    if (headers === undefined) {
      headers = { 'Content-Type': 'application/json' };
    }
    const _headers = await this.makeHeaders(headers);
    const response = await fetch(`${API_BASE_URL}/${sub_path}/${resource}/`, {
      method: this.POST,
      body: JSON.stringify(data),
      headers: _headers, // await this.makeHeaders({ "Content-Type": 'application/json'})
    });
    // error if request failed
    if (response.status !== 201) {
      throw await this.createHttpError(response);
    }
    return await response.json();
  }

  async patchResource(
    resource: string,
    data: object,
    id: number,
  ): Promise<Response> {
    const response = await fetch(
      `${API_BASE_URL}/${ASSETVERWALTUNG_SUBPATH}/${resource}/${id}/`,
      {
        method: this.PATCH,
        body: JSON.stringify(data),
        headers: await this.makeHeaders({ 'Content-Type': 'application/json' }),
      },
    );
    // error if request failed
    if (response.status !== 200) {
      throw await this.createHttpError(response);
    }
    return await response.json();
  }

  async patchxResource(
    subpath: string,
    resource: string,
    data: object,
    id: number,
  ): Promise<object> {
    const response = await fetch(
      `${API_BASE_URL}/${subpath}/${resource}/${id}/`,
      {
        method: this.PATCH,
        body: JSON.stringify(data),
        headers: await this.makeHeaders({ 'Content-Type': 'application/json' }),
      },
    );
    // error if request failed
    if (response.status !== 200) {
      throw await this.createHttpError(response);
    }
    return await response.json();
  }

  async deletexResource(
    subpath: string,
    resource: string,
    id: number,
  ): Promise<void> {
    const response = await fetch(
      `${API_BASE_URL}/${subpath}/${resource}/${id}/`,
      {
        method: this.DELETE,
        headers: await this.makeHeaders(),
      },
    );
    // error if request failed
    if (response.status !== 204) {
      throw new Error(
        `request failed with status ${response.status}, reason: ${response.statusText}`,
      );
    }
  }
  async deleteResource(resource: string, id: number): Promise<void> {
    const response = await fetch(
      `${API_BASE_URL}/${ASSETVERWALTUNG_SUBPATH}/${resource}/${id}/`,
      {
        method: this.DELETE,
        headers: await this.makeHeaders(),
      },
    );
    // error if request failed
    if (response.status !== 204) {
      throw new Error(
        `request failed with status ${response.status}, reason: ${response.statusText}`,
      );
    }
  }

  // Media helpers. hochgeladen_von is not sent (the backend falls back to "Anonymous").
  async getMediaContentAsBlobURL(id: number): Promise<string> {
    const response = await fetch(`${API_BASE_URL}/bwd/Media/${id}/content/`, {
      method: 'GET',
      headers: await this.makeHeaders(),
    });

    // Handle errors
    if (!response.ok) {
      throw new Error(
        `Request failed with status ${response.status}, reason: ${response.statusText}`,
      );
    }

    // Convert response to a Blob
    const blob = await response.blob();

    // Create a Blob URL
    const blobUrl = URL.createObjectURL(blob);
    return blobUrl;
  }

  async uploadMedia(subpath: string, data: any): Promise<any> {
    const formData = new FormData();
    // Füge alle Daten zum FormData hinzu
    formData.append('name', data.name);
    if (data.beschreibung) formData.append('beschreibung', data.beschreibung);
    if (data.local_id) formData.append('local_id', String(data.local_id));
    if (data.datei) formData.append('datei', data.datei); // Datei hinzufügen
    if (data.hochgeladen_von)
      formData.append('hochgeladen_von', data.hochgeladen_von);
    formData.append('model_name', data.model_name);
    formData.append('model_pk', String(data.model_pk));
    if (data.projekt) formData.append('projekt', String(data.projekt));
    if (data.type) formData.append('type', data.type);
    if (data.media_type) formData.append('media_type', data.media_type);
    if (data.bauwerk) formData.append('bauwerk', String(data.bauwerk));

    const response = await fetch(`${API_BASE_URL}/${subpath}/Media/`, {
      method: 'POST',
      body: formData,
      headers: await this.makeHeaders(),
    });

    if (response.status !== 200 && response.status !== 201) {
      throw await this.createHttpError(response);
    }

    return await response.json();
  }

  async deleteMedia(id: number): Promise<boolean> {
    const response = await fetch(`${API_BASE_URL}/bwd/Media/${id}/`, {
      method: 'DELETE',
      headers: await this.makeHeaders(),
    });

    // response.ok is true if status is in the range 200-299
    return response.ok;
  }

  /**
   * Get detailed information for a media item
   */
  async getMediaDetails(app: string, mediaId: number | string): Promise<any> {
    const response = await fetch(`${API_BASE_URL}/${app}/Media/${mediaId}/`, {
      method: this.GET,
      headers: await this.makeHeaders(),
    });

    if (!response.ok) {
      throw new Error(
        `request failed with status ${response.status}, reason: ${response.statusText}`,
      );
    }

    return await response.json();
  }

  /**
   * Retrieve multiple media objects in a single request (batch endpoint)
   */
  async getMediaBatch(ids: number[]): Promise<any[]> {
    if (!ids || ids.length === 0) return [];
    const response = await fetch(`${API_BASE_URL}/bwd/Media/batch/`, {
      method: this.POST,
      body: JSON.stringify({ ids }),
      headers: await this.makeHeaders({ 'Content-Type': 'application/json' }),
    });

    if (!response.ok) {
      throw new Error(
        `batch request failed with status ${response.status}, reason: ${response.statusText}`,
      );
    }

    return await response.json();
  }

  /**
   * Update media details
   */
  async updateMediaDetails(
    app: string,
    mediaId: number,
    data: any,
  ): Promise<any> {
    const response = await fetch(`${API_BASE_URL}/${app}/Media/${mediaId}/`, {
      method: this.PATCH,
      body: JSON.stringify(data),
      headers: await this.makeHeaders({ 'Content-Type': 'application/json' }),
    });

    if (!response.ok) {
      throw new Error(
        `request failed with status ${response.status}, reason: ${response.statusText}`,
      );
    }

    return await response.json();
  }

  /**
   * Upload media with structure information
   */
  async uploadMediaWithStructureInfo(data: any): Promise<any> {
    const formData = new FormData();
    if (data.name) {
      formData.append('name', data.name);
    }
    formData.append('media_type', data.media_type);
    formData.append('file', data.file);
    formData.append('structure_type', data.structure_type);
    formData.append('structure_id', data.structure_id.toString());
    formData.append('bauwerk', String(data.bauwerk));

    const response = await fetch(`${API_BASE_URL}/bwd/upload-media/`, {
      method: this.POST,
      body: formData,
      headers: await this.makeHeaders(),
    });

    if (!response.ok) {
      throw new Error(
        `Upload failed with status ${response.status}, reason: ${response.statusText}`,
      );
    }

    return await response.json();
  }

  /**
   * Upload multiple media files with structure information (batch upload)
   */
  async batchUploadMedia(data: any): Promise<any> {
    const formData = new FormData();
    formData.append('structure_type', data.structure_type);
    formData.append('structure_id', data.structure_id.toString());
    formData.append('media_type', data.media_type);
    formData.append('bauwerk', String(data.bauwerk));

    if (data.name) {
      formData.append('name', data.name);
    }

    if (Array.isArray(data.files)) {
      data.files.forEach((file: File) => {
        formData.append('files', file);
      });
    }

    const response = await fetch(
      `${API_BASE_URL}/bwd/upload-media/batch-upload/`,
      {
        method: this.POST,
        body: formData,
        headers: await this.makeHeaders(),
      },
    );

    if (!response.ok) {
      throw new Error(
        `Batch upload failed with status ${response.status}, reason: ${response.statusText}`,
      );
    }

    return await response.json();
  }

  /**
   * Universal function to get data from the backend. The `utype` parameter is used to specify the API subpath.
   *
   * @template T - The type of the response data.
   * @param {string} utype - The type of the investigation procedure.
   * @param {number} id - The ID of the investigation procedure.
   * @returns {Promise<T>} - A promise that resolves to the response data of type T.
   * @throws {Error} - Throws an error if the request fails with a status other than 200.
   */
  async getUntersuchungsverfahrenDetail<T>(
    utype: string,
    id: number,
  ): Promise<T> {
    const response = await fetch(
      `${API_BASE_URL}/${DIAGNOSTIK_SUBPATH}/Untersuchungsverfahren/${id}/?Untersuchung=${utype}`,
      {
        method: this.GET,
        headers: await this.makeHeaders(),
      },
    );

    // error if request failed
    if (response.status !== 200) {
      throw new Error(
        `request failed with status ${response.status}, reason: ${response.statusText}`,
      );
    }
    return response.json();
  }

  /**
   * Sends a PATCH request to update the details of a specific "Untersuchungsverfahren"
   *
   * @param utype - The type of the investigation procedure.
   * @param id - The ID of the investigation procedure to be updated.
   * @param data - The data to be sent in the body of the PATCH request.
   * @returns A promise that resolves to the response data in JSON format.
   * @throws Will throw an error if the request fails with a status other than 200.
   */
  async patchUntersuchungsverfahrenDetail(
    utype: string,
    id: number,
    data: any,
  ): Promise<any> {
    const response = await fetch(
      `${API_BASE_URL}/${DIAGNOSTIK_SUBPATH}/Untersuchungsverfahren/${id}/?Untersuchung=${utype}`,
      {
        method: this.PATCH,
        body: JSON.stringify(data),
        headers: await this.makeHeaders({ 'Content-Type': 'application/json' }),
      },
    );

    // error if request failed
    if (response.status !== 200) {
      throw new Error(
        `request failed with status ${response.status}, reason: ${response.statusText}`,
      );
    }
    return response.json();
  }

  /**
   * GET list of a specific Untersuchungsverfahren type.
   */
  async getUntersuchungsverfahrenList<T>(utype: string): Promise<T[]> {
    const url = new URL(`${API_BASE_URL}/${DIAGNOSTIK_SUBPATH}/Untersuchungsverfahren/`);
    url.searchParams.append('Untersuchung', utype);
    const routeBauwerkId = router.currentRoute.value.params.bauwerkId;
    if (routeBauwerkId) {
      url.searchParams.append('bauwerk', String(routeBauwerkId));
    }
    const response = await fetch(url.toString(), {
      method: this.GET,
      headers: await this.makeHeaders(),
    });
    if (!response.ok) {
      throw new Error(`fetch list failed ${response.status}`);
    }
    const json = (await response.json()) as Record<string, T[]>;
    return json[utype] || [];
  }

  /**
   * POST (create) a new Untersuchungsverfahren of given type.
   */
  async postUntersuchungsverfahren<T>(utype: string, data: object): Promise<T> {
    const url = new URL(`${API_BASE_URL}/${DIAGNOSTIK_SUBPATH}/Untersuchungsverfahren/`);
    url.searchParams.append('Untersuchung', utype);
    const response = await fetch(url.toString(), {
      method: this.POST,
      headers: await this.makeHeaders({ 'Content-Type': 'application/json' }),
      body: JSON.stringify(data),
    });
    if (response.status !== 201) {
      throw await this.createHttpError(response);
    }
    return await response.json();
  }

  /**
   * PATCH (update) a Untersuchungsverfahren of given type and id.
   */
  async patchUntersuchungsverfahren<T>(
    utype: string,
    id: number | string,
    data: object,
  ): Promise<T> {
    const url = new URL(`${API_BASE_URL}/${DIAGNOSTIK_SUBPATH}/Untersuchungsverfahren/${id}/`);
    url.searchParams.append('Untersuchung', utype);
    const response = await fetch(url.toString(), {
      method: this.PATCH,
      headers: await this.makeHeaders({ 'Content-Type': 'application/json' }),
      body: JSON.stringify(data),
    });
    if (!response.ok) {
      throw await this.createHttpError(response);
    }
    return await response.json();
  }

  /**
   * DELETE a Untersuchungsverfahren of given type and id.
   */
  async deleteUntersuchungsverfahren(
    utype: string,
    id: number | string,
  ): Promise<void> {
    const url = new URL(`${API_BASE_URL}/${DIAGNOSTIK_SUBPATH}/Untersuchungsverfahren/${id}/`);
    url.searchParams.append('Untersuchung', utype);
    const response = await fetch(url.toString(), {
      method: this.DELETE,
      headers: await this.makeHeaders(),
    });
    if (response.status !== 204) {
      throw new Error(`delete failed ${response.status}`);
    }
  }

  BWD = {
    /**
     * Fetches UV data based on the specified art, bauwerk, and projekt, with optional filters.
     *
     * @param art - The type of UV data to fetch (e.g., "VorOrtUntersuchung"). This determines the endpoint used.
     * @param bauwerk - Optional. The bauwerk to filter the data by.
     * @param projekt - Optional. The projekt to filter the data by.
     * @param filters - Optional. An object containing additional filters to apply to the query.
     * @returns A promise that resolves to the fetched UV data as a JSON object.
     * @throws Error if the fetch operation fails.
     */
    async fetchUvData(
      art: string,
      bauwerk?: string,
      projekt?: string,
      filters: object = {},
    ): Promise<any> {
      // Compose query params
      const params: any = { ...filters };
      if (bauwerk) params.bauwerk = bauwerk;
      if (projekt) params.projekt = projekt;
      params.art = art;

      // Use makeQueryParams to build the query string
      const queryString = apiConnector.makeQueryParams(false, params);

      // Fetch from the correct endpoint
      const response = await fetch(
        `${API_BASE_URL}/${DIAGNOSTIK_SUBPATH}/Untersuchungsverfahren/${queryString}`,
        {
          method: 'GET',
          headers: await apiConnector.makeHeaders(),
        },
      );
      if (!response.ok) throw new Error(`Failed to fetch ${art} data`);
      return await response.json();
    },
    async fetchVorOrtUntersuchungData(
      bauwerk?: string,
      projekt?: string,
      selectedUntersuchungsverfahrenOptions?: string[],
    ): Promise<any> {
      const url = new URL('api/bwd/Untersuchungsverfahren/', API_BASE_URL);
      url.searchParams.append('format', 'json');
      url.searchParams.append('art', 'VorOrtUntersuchung');

      if (selectedUntersuchungsverfahrenOptions) {
        const formattedOptions =
          selectedUntersuchungsverfahrenOptions.join(',');

        url.searchParams.append('Untersuchung', formattedOptions);
      }
      if (bauwerk) {
        url.searchParams.append('bauwerk', bauwerk.toString());
      }
      if (projekt) {
        url.searchParams.append('projekt', projekt.toString());
      }

      const response = await fetch(url.toString(), {
        headers: await apiConnector.makeHeaders({
          'Content-Type': 'application/json',
        }),
      });
      if (!response.ok) {
        throw new Error('Fehler beim Laden der Vor-Ort-Untersuchungen');
      }

      return response.json();
    },

    async fetchLaboruntersuchungData(
      bauwerk?: string,
      projekt?: string,
      selectedUntersuchungsverfahrenOptions?: string[],
    ): Promise<any> {
      const url = new URL('api/bwd/Untersuchungsverfahren/', API_BASE_URL);
      url.searchParams.append('format', 'json');
      url.searchParams.append('art', 'Laboruntersuchung');

      if (selectedUntersuchungsverfahrenOptions) {
        const formattedOptions =
          selectedUntersuchungsverfahrenOptions.join(',');
        url.searchParams.append('Untersuchung', formattedOptions);
      }
      if (bauwerk) {
        url.searchParams.append('bauwerk', bauwerk.toString());
      }
      if (projekt) {
        url.searchParams.append('projekt', projekt.toString());
      }

      const response = await fetch(url.toString(), {
        headers: await apiConnector.makeHeaders({
          'Content-Type': 'application/json',
        }),
      });
      if (!response.ok) {
        throw new Error('Fehler beim Laden der Laboruntersuchung-Daten');
      }

      return response.json();
    },

    /**
     * Fetches detailed data for a specific Untersuchung (examination) by its ID.
     *
     * @param id - The ID of the Untersuchung to fetch details for.
     * @param untersuchung - The name or type of the Untersuchung.
     * @returns A promise that resolves to the detailed data of the Untersuchung.
     * @throws Will throw an error if the response is not ok.
     */
    async fetchUntersuchungDetailData(
      id: number,
      untersuchung: string,
      art?: string,
    ): Promise<any> {
      const url = new URL(
        `api/bwd/Untersuchungsverfahren/${id}/`,
        API_BASE_URL,
      );
      url.searchParams.append('Untersuchung', getMethodApiName(untersuchung));
      if (art) url.searchParams.append('art', art);

      const response = await fetch(url.toString(), {
        headers: await apiConnector.makeHeaders(),
      });
      if (!response.ok) {
        throw new Error('Fehler beim Laden der Detaildaten');
      }
      return response.json();
    },

    /**
     * Deletes the specified Untersuchung data by its ID.
     *
     * @param id - The ID of the Untersuchung to delete.
     * @param untersuchung - The name of the Untersuchung to delete.
     * @returns A promise that resolves when the deletion is complete.
     * @throws Will throw an error if the deletion fails.
     */
    async deleteUntersuchungData(
      id: number,
      untersuchung: string,
    ): Promise<void> {
      const url = new URL(
        `api/bwd/Untersuchungsverfahren/${id}/`,
        API_BASE_URL,
      );
      url.searchParams.append('Untersuchung', getMethodApiName(untersuchung));

      const response = await fetch(url.toString(), {
        method: 'DELETE',
        headers: await apiConnector.makeHeaders(),
      });

      if (!response.ok) {
        throw new Error('Fehler beim Löschen der Untersuchung');
      }
    },

    /**
     * Fetches material data from the API.
     *
     * @returns {Promise<any>} A promise that resolves to the material data.
     * @throws {Error} Throws an error if the request fails.
     */
    async fetchMaterialData(): Promise<any> {
      const url = new URL(`api/bwd/Material/`, API_BASE_URL);

      const response = await fetch(url.toString(), {
        headers: await apiConnector.makeHeaders(),
      });
      if (!response.ok) {
        throw new Error('Fehler beim Laden der Materialdaten');
      }
      return response.json();
    },

    /**
     * Fetches the Bearbeiter data from the API.
     *
     * @returns {Promise<any>} A promise that resolves to the Bearbeiter data.
     * @throws {Error} Throws an error if there is an issue with the fetch request.
     */
    async fetchBearbeiterData(): Promise<any> {
      const url = new URL(`api/bwd/Bearbeiter/`, API_BASE_URL);

      const response = await fetch(url.toString(), {
        headers: await apiConnector.makeHeaders(),
      });
      if (!response.ok) {
        throw new Error('Fehler beim Laden der Bearbeiterdaten');
      }
      return response.json();
    },

    /**
     * Fetches Bohrkern data from the API.
     *
     * @param {number} id - The ID of the Bohrkern to fetch.
     * @returns {Promise<any>} A promise that resolves to the Bohrkern data.
     * @throws {Error} Throws an error if the data could not be loaded.
     */
    async fetchBohrkernData(id: number): Promise<any> {
      const url = new URL(`api/bwd/Bohrkern/${id}/`, API_BASE_URL);
      const response = await fetch(url.toString(), {
        headers: await apiConnector.makeHeaders(),
      });
      if (!response.ok) {
        throw new Error('Fehler beim Laden der Bohrkern-Daten');
      }
      return response.json();
    },
    async fetchProbeData(id: number): Promise<any> {
      const url = new URL(`api/bwd/Probe/${id}/`, API_BASE_URL);
      const response = await fetch(url.toString(), {
        headers: await apiConnector.makeHeaders(),
      });
      if (!response.ok) {
        throw new Error('Fehler beim Laden der Probe-Daten');
      }
      return response.json();
    },

    async createUntersuchung(untersuchung: string, data: any): Promise<any> {
      const url = new URL(`api/bwd/Untersuchungsverfahren/`, API_BASE_URL);

      data.type = getMethodApiName(untersuchung);
      const response = await fetch(url.toString(), {
        method: 'POST',
        body: JSON.stringify(data),
        headers: await apiConnector.makeHeaders({
          'Content-Type': 'application/json',
        }),
      });

      if (!response.ok) {
        throw await apiConnector.createHttpError(response);
      }
      return response.json();
    },

    async updateUntersuchung(
      id: number,
      untersuchung: string,
      data: any,
    ): Promise<any> {
      const url = new URL(
        `api/bwd/Untersuchungsverfahren/${id}/`,
        API_BASE_URL,
      );
      url.searchParams.append('Untersuchung', getMethodApiName(untersuchung));

      const response = await fetch(url.toString(), {
        method: 'PATCH',
        body: JSON.stringify(data),
        headers: await apiConnector.makeHeaders({
          'Content-Type': 'application/json',
        }),
      });

      if (!response.ok) {
        throw await apiConnector.createHttpError(response);
      }
      return response.json();
    },
  };
}

const apiConnector = new ApiConnector();
export { API_BASE_URL, apiConnector };
