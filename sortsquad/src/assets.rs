use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct ImageAssets {
    #[asset(path = "bins/general/static.png")]
    pub bin_general: Handle<Image>,

    #[asset(path = "bins/recyclable/static.png")]
    pub bin_recycle: Handle<Image>,

    #[asset(path = "bins/organic/static.png")]
    pub bin_organic: Handle<Image>,

    #[asset(path = "bins/hazardous/static.png")]
    pub bin_hazardous: Handle<Image>,

    // General trashes
    
    #[asset(path = "trashes/general/ciggarette.png")]
    pub trash_ciggarette: Handle<Image>,

    #[asset(path = "trashes/general/shoe.png")]
    pub trash_shoe: Handle<Image>,

    #[asset(path = "trashes/general/tissue.png")]
    pub trash_tissue: Handle<Image>,

    // Recyclable trashes

    #[asset(path = "trashes/recyclable/coke.png")]
    pub trash_coke: Handle<Image>,

    #[asset(path = "trashes/recyclable/newspaper.png")]
    pub trash_newspaper: Handle<Image>,

    #[asset(path = "trashes/recyclable/waterbottle.png")]
    pub trash_waterbottle: Handle<Image>,

    // Organic trashes 
    
    #[asset(path = "trashes/organic/apple.png")]
    pub trash_apple: Handle<Image>,

    #[asset(path = "trashes/organic/fishbone.png")]
    pub trash_fishbone: Handle<Image>,

    #[asset(path = "trashes/organic/vegatable.png")]
    pub trash_vegatable: Handle<Image>,

    // Hazardous trashes 

    #[asset(path = "trashes/hazardous/battery.png")]
    pub trash_battery: Handle<Image>,

    #[asset(path = "trashes/hazardous/bleach.png")]
    pub trash_bleach: Handle<Image>,

    #[asset(path = "trashes/hazardous/electronic.png")]
    pub trash_electronic: Handle<Image>,

    // Controls labels
    
    #[asset(path = "ui/controls/a_d.png")]
    pub control_a_d: Handle<Image>,

    #[asset(path = "ui/controls/g_h.png")]
    pub control_g_h: Handle<Image>,

    #[asset(path = "ui/controls/bl_br.png")]
    pub control_bl_br: Handle<Image>,

    #[asset(path = "ui/controls/al_ar.png")]
    pub control_al_ar: Handle<Image>,

    // Cutscene
 
    #[asset(path = "cutscene/1.png")]
    pub cutscene_1: Handle<Image>,

    #[asset(path = "cutscene/2.png")]
    pub cutscene_2: Handle<Image>,

    #[asset(path = "cutscene/3.png")]
    pub cutscene_3: Handle<Image>,

    #[asset(path = "cutscene/4.png")]
    pub cutscene_4: Handle<Image>,

    #[asset(path = "cutscene/5.png")]
    pub cutscene_5: Handle<Image>,

    #[asset(path = "cutscene/6.png")]
    pub cutscene_6: Handle<Image>,

    #[asset(path = "cutscene/7.png")]
    pub cutscene_7: Handle<Image>,

    #[asset(path = "cutscene/8.png")]
    pub cutscene_8: Handle<Image>,

    #[asset(path = "cutscene/9.png")]
    pub cutscene_9: Handle<Image>,

    #[asset(path = "cutscene/10.png")]
    pub cutscene_10: Handle<Image>,

    #[asset(path = "cutscene/11.png")]
    pub cutscene_11: Handle<Image>,

    #[asset(path = "cutscene/12.png")]
    pub cutscene_12: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct FontAssets {
    #[asset(path = "ui/fonts/font.otf")]
    pub font: Handle<Font>,
}
