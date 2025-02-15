use ggez::audio::{SoundData, SoundSource, Source};
use ggez::Context;

pub struct Audio {
    deal: SoundData,
    drop: SoundData,
    take: SoundData,
    pub shuffle: Source,
}

impl Audio {
    pub fn new(ctx: &mut Context) -> Self {
        let deal = SoundData::new(ctx, "/deal.wav").unwrap();
        let drop = SoundData::new(ctx, "/drop.wav").unwrap();
        let take = SoundData::new(ctx, "/take.wav").unwrap();
        let mut shuffle = Source::from_data(ctx, take.clone()).unwrap();
        shuffle.set_repeat(true);
        shuffle.set_pitch(4.);
        shuffle.set_volume(0.6);

        Self {
            deal,
            drop,
            take,
            shuffle,
        }
    }
    pub fn play_deal(&self, ctx: &mut Context) {
        let mut source = Source::from_data(ctx, self.deal.clone()).unwrap();
        source.set_volume(0.1);
        source.play_detached(ctx).unwrap();
    }
    pub fn play_drop(&self, ctx: &mut Context) {
        let mut source = Source::from_data(ctx, self.drop.clone()).unwrap();
        source.set_volume(0.1);
        source.set_pitch(1.7);
        source.play_detached(ctx).unwrap();
    }
    pub fn play_take(&self, ctx: &mut Context) {
        let mut source = Source::from_data(ctx, self.take.clone()).unwrap();
        source.set_volume(0.1);
        source.set_pitch(1.7);
        source.play_detached(ctx).unwrap();
    }
}
