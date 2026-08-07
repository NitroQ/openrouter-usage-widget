export type ReleaseAsset = {
  name: string;
  downloadUrl: string;
  size: number;
  signatureUrl: string;
  sha256?: string;
};

export type UpdateInfo = {
  currentVersion: string;
  latestVersion: string;
  updateAvailable: boolean;
  releaseUrl: string;
  assets: ReleaseAsset[];
  releaseTag: string;
  compatibleAssetAvailable: boolean;
  asset: ReleaseAsset | null;
};

export type UpdateStatus = {
  currentVersion: string;
  lastCheckAt: string | null;
  updateAvailable: boolean;
  latestVersion: string | null;
  compatibleAssetAvailable: boolean;
  asset: ReleaseAsset | null;
};
