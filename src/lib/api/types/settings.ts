export type ProfileSelectionLayoutMode = 'grid' | 'list';

export function toggleLayoutMode(
    currentMode: ProfileSelectionLayoutMode
): ProfileSelectionLayoutMode {
    return currentMode === 'grid' ? 'list' : 'grid';
}
