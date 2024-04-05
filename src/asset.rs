use sb_sbity::asset::{Asset, Costume, Sound};

use crate::resource::Resource;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CostumeBuilder {
    pub rotation_center_x: i64,
    pub rotation_center_y: i64,
    pub asset: AssetBuilder,
}

impl CostumeBuilder {
    pub fn new(asset_builder: AssetBuilder) -> CostumeBuilder {
        CostumeBuilder {
            asset: asset_builder,
            rotation_center_x: 0,
            rotation_center_y: 0,
        }
    }

    pub fn set_rotation_center(&mut self, x: i64, y: i64) -> &mut Self {
        self.rotation_center_x = x;
        self.rotation_center_y = y;
        self
    }

    pub fn rotation_center_x(&mut self, x: i64) -> &mut Self {
        self.rotation_center_x = x;
        self
    }

    pub fn rotation_center_y(&mut self, y: i64) -> &mut Self {
        self.rotation_center_y = y;
        self
    }

    pub fn asset(&mut self, asset: AssetBuilder) -> &mut Self {
        self.asset = asset;
        self
    }

    pub fn build(self, file_buff: &mut Vec<Resource>) -> Costume {
        let CostumeBuilder {
            rotation_center_x,
            rotation_center_y,
            asset,
        } = self;
        Costume {
            rotation_center_x: rotation_center_x.into(),
            rotation_center_y: rotation_center_y.into(),
            bitmap_resolution: Some(1),
            asset: asset.build(file_buff),
        }
    }
}

/// Not really sure what to do here yet
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SoundBuilder {
    pub rate: u64,
    pub sample_count: u64,
    pub format: Option<String>,
    pub asset: AssetBuilder,
}

impl SoundBuilder {
    pub fn rate(&mut self, rate: u64) -> &mut Self {
        self.rate = rate;
        self
    }

    pub fn sample_count(&mut self, sample_count: u64) -> &mut Self {
        self.sample_count = sample_count;
        self
    }

    pub fn format(&mut self, format: Option<String>) -> &mut Self {
        self.format = format;
        self
    }

    pub fn asset(&mut self, asset: AssetBuilder) -> &mut Self {
        self.asset = asset;
        self
    }

    pub fn build(self, file_buff: &mut Vec<Resource>) -> Sound {
        let SoundBuilder {
            rate,
            sample_count,
            format,
            asset,
        } = self;
        Sound {
            rate,
            sample_count,
            format,
            asset: asset.build(file_buff),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssetBuilder {
    pub name: String,
    pub resource: Resource,
}

impl AssetBuilder {
    pub fn new<S: Into<String>>(name: S, resource: Resource) -> AssetBuilder {
        AssetBuilder {
            name: name.into(),
            resource,
        }
    }

    pub fn build(self, res_buf: &mut Vec<Resource>) -> Asset {
        let AssetBuilder { name, mut resource } = self;
        let extension = resource.extension().to_owned();
        let md5_hash = resource.get_or_compute_md5_hash();
        let asset = Asset {
            asset_id: md5_hash.to_owned(),
            name,
            md5ext: Some(md5_hash.to_owned() + "." + &extension),
            data_format: extension,
        };
        res_buf.push(resource);
        asset
    }
}
