#[doc = "Register `SHRR` reader"]
pub type R = crate::R<ShrrSpec>;
#[doc = "Register `SHRR` writer"]
pub type W = crate::W<ShrrSpec>;
#[doc = "Field `TREFRESH1` reader - DAC Channel 1 refresh Time (only valid in sample &amp; hold mode) Refresh time= (TREFRESH\\[7:0\\]) x T LSI"]
pub type Trefresh1R = crate::FieldReader;
#[doc = "Field `TREFRESH1` writer - DAC Channel 1 refresh Time (only valid in sample &amp; hold mode) Refresh time= (TREFRESH\\[7:0\\]) x T LSI"]
pub type Trefresh1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TREFRESH2` reader - DAC Channel 2 refresh Time (only valid in sample &amp; hold mode) Refresh time= (TREFRESH\\[7:0\\]) x T LSI"]
pub type Trefresh2R = crate::FieldReader;
#[doc = "Field `TREFRESH2` writer - DAC Channel 2 refresh Time (only valid in sample &amp; hold mode) Refresh time= (TREFRESH\\[7:0\\]) x T LSI"]
pub type Trefresh2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - DAC Channel 1 refresh Time (only valid in sample &amp; hold mode) Refresh time= (TREFRESH\\[7:0\\]) x T LSI"]
    #[inline(always)]
    pub fn trefresh1(&self) -> Trefresh1R {
        Trefresh1R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - DAC Channel 2 refresh Time (only valid in sample &amp; hold mode) Refresh time= (TREFRESH\\[7:0\\]) x T LSI"]
    #[inline(always)]
    pub fn trefresh2(&self) -> Trefresh2R {
        Trefresh2R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DAC Channel 1 refresh Time (only valid in sample &amp; hold mode) Refresh time= (TREFRESH\\[7:0\\]) x T LSI"]
    #[inline(always)]
    pub fn trefresh1(&mut self) -> Trefresh1W<ShrrSpec> {
        Trefresh1W::new(self, 0)
    }
    #[doc = "Bits 16:23 - DAC Channel 2 refresh Time (only valid in sample &amp; hold mode) Refresh time= (TREFRESH\\[7:0\\]) x T LSI"]
    #[inline(always)]
    pub fn trefresh2(&mut self) -> Trefresh2W<ShrrSpec> {
        Trefresh2W::new(self, 16)
    }
}
#[doc = "DAC Sample and Hold refresh time register\n\nYou can [`read`](crate::Reg::read) this register and get [`shrr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shrr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ShrrSpec;
impl crate::RegisterSpec for ShrrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`shrr::R`](R) reader structure"]
impl crate::Readable for ShrrSpec {}
#[doc = "`write(|w| ..)` method takes [`shrr::W`](W) writer structure"]
impl crate::Writable for ShrrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SHRR to value 0x0001_0001"]
impl crate::Resettable for ShrrSpec {
    const RESET_VALUE: u32 = 0x0001_0001;
}
