import { mdiFile, mdiFileDelimited, mdiFileExcel, mdiFileImage, mdiFilePdfBox, mdiFileWord } from '@mdi/js';

// Shared helpers of MultiFileField/SingleFileField. File types are shown with mdi icons.

export function getFileExtension(dateiUrl: string | null | undefined): string {
    if (!dateiUrl) return '';
    // Try URL parsing first
    try {
        const url = new URL(dateiUrl);
        const pathname = url.pathname;
        const ext = pathname.split('.').pop()?.toLowerCase() || '';
        if (ext && !ext.includes('/')) return ext.split(/[#?]/)[0];
    } catch {
        // Ignore parsing errors
    }
    // Fallback: extract from string directly (handles relative paths & query params)
    const cleaned = dateiUrl.split('/').pop() || dateiUrl;
    const extMatch = cleaned.toLowerCase().split(/[#?]/)[0].split('.').pop();
    return extMatch && extMatch !== cleaned.toLowerCase() ? extMatch : '';
}

export function isImageFile(dateiUrl: string | null | undefined): boolean {
    const imageExtensions = ['jpg', 'jpeg', 'png', 'gif', 'bmp', 'webp', 'svg'];
    const extension = getFileExtension(dateiUrl);
    return imageExtensions.includes(extension);
}

export function getFileIcon(dateiUrl: string | null | undefined): string {
    const type = getFileExtension(dateiUrl);
    if (type === 'docx' || type === 'doc') {
        return mdiFileWord;
    } else if (type === 'xlsx' || type === 'xls') {
        return mdiFileExcel;
    } else if (type === 'csv' || type === 'txt') {
        return mdiFileDelimited;
    } else if (type === 'pdf') {
        return mdiFilePdfBox;
    } else if (isImageFile(dateiUrl)) {
        return mdiFileImage;
    }
    return mdiFile;
}

export function formatDate(value: string | null | undefined): string {
    if (!value) return '';
    const date = new Date(value);
    return Number.isNaN(date.getTime()) ? value : date.toLocaleString('de-DE');
}
