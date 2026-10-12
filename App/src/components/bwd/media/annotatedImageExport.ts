import { apiConnector } from '@/utils/ApiConnector';

// No window.open fallback (new windows are blocked in the Tauri app); errors are rethrown.

export function downloadBlob(blob: Blob, filename: string) {
	const url = URL.createObjectURL(blob);
	const a = document.createElement('a');
	a.href = url;
	a.download = filename;
	document.body.appendChild(a);
	a.click();
	document.body.removeChild(a);
	URL.revokeObjectURL(url);
}

/**
 * Download the untouched original media file (no annotations), keeping its
 * original file extension.
 */
export async function downloadMediaOriginal(mediaId: number, name: string | undefined, originalUrl: string): Promise<void> {
	try {
		const blobUrl = await apiConnector.getMediaContentAsBlobURL(mediaId);
		const extension = originalUrl.match(/\.([a-z0-9]+)(?:\?|$)/i)?.[1] ?? 'jpg';
		const base = (name || `bild-${mediaId}`).replace(/\.[a-z0-9]+$/i, '');

		const a = document.createElement('a');
		a.href = blobUrl;
		a.download = `${base}.${extension}`;
		document.body.appendChild(a);
		a.click();
		document.body.removeChild(a);
		// Revoke after the click was handled, otherwise WebView2 may abort the download.
		setTimeout(() => URL.revokeObjectURL(blobUrl), 1000);
	} catch (error) {
		console.error('Original-Download fehlgeschlagen', error);
		throw error;
	}
}
