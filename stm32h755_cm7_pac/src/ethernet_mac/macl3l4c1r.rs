#[doc = "Register `MACL3L4C1R` reader"]
pub type R = crate::R<Macl3l4c1rSpec>;
#[doc = "Register `MACL3L4C1R` writer"]
pub type W = crate::W<Macl3l4c1rSpec>;
#[doc = "Field `L3PEN1` reader - L3PEN1"]
pub type L3pen1R = crate::BitReader;
#[doc = "Field `L3PEN1` writer - L3PEN1"]
pub type L3pen1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L3SAM1` reader - L3SAM1"]
pub type L3sam1R = crate::BitReader;
#[doc = "Field `L3SAM1` writer - L3SAM1"]
pub type L3sam1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L3SAIM1` reader - L3SAIM1"]
pub type L3saim1R = crate::BitReader;
#[doc = "Field `L3SAIM1` writer - L3SAIM1"]
pub type L3saim1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L3DAM1` reader - L3DAM1"]
pub type L3dam1R = crate::BitReader;
#[doc = "Field `L3DAM1` writer - L3DAM1"]
pub type L3dam1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L3DAIM1` reader - L3DAIM1"]
pub type L3daim1R = crate::BitReader;
#[doc = "Field `L3DAIM1` writer - L3DAIM1"]
pub type L3daim1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L3HSBM1` reader - L3HSBM1"]
pub type L3hsbm1R = crate::FieldReader;
#[doc = "Field `L3HSBM1` writer - L3HSBM1"]
pub type L3hsbm1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `L3HDBM1` reader - L3HDBM1"]
pub type L3hdbm1R = crate::FieldReader;
#[doc = "Field `L3HDBM1` writer - L3HDBM1"]
pub type L3hdbm1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `L4PEN1` reader - L4PEN1"]
pub type L4pen1R = crate::BitReader;
#[doc = "Field `L4PEN1` writer - L4PEN1"]
pub type L4pen1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L4SPM1` reader - L4SPM1"]
pub type L4spm1R = crate::BitReader;
#[doc = "Field `L4SPM1` writer - L4SPM1"]
pub type L4spm1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L4SPIM1` reader - L4SPIM1"]
pub type L4spim1R = crate::BitReader;
#[doc = "Field `L4SPIM1` writer - L4SPIM1"]
pub type L4spim1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L4DPM1` reader - L4DPM1"]
pub type L4dpm1R = crate::BitReader;
#[doc = "Field `L4DPM1` writer - L4DPM1"]
pub type L4dpm1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L4DPIM1` reader - L4DPIM1"]
pub type L4dpim1R = crate::BitReader;
#[doc = "Field `L4DPIM1` writer - L4DPIM1"]
pub type L4dpim1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - L3PEN1"]
    #[inline(always)]
    pub fn l3pen1(&self) -> L3pen1R {
        L3pen1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - L3SAM1"]
    #[inline(always)]
    pub fn l3sam1(&self) -> L3sam1R {
        L3sam1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - L3SAIM1"]
    #[inline(always)]
    pub fn l3saim1(&self) -> L3saim1R {
        L3saim1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - L3DAM1"]
    #[inline(always)]
    pub fn l3dam1(&self) -> L3dam1R {
        L3dam1R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - L3DAIM1"]
    #[inline(always)]
    pub fn l3daim1(&self) -> L3daim1R {
        L3daim1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:10 - L3HSBM1"]
    #[inline(always)]
    pub fn l3hsbm1(&self) -> L3hsbm1R {
        L3hsbm1R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bits 11:15 - L3HDBM1"]
    #[inline(always)]
    pub fn l3hdbm1(&self) -> L3hdbm1R {
        L3hdbm1R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bit 16 - L4PEN1"]
    #[inline(always)]
    pub fn l4pen1(&self) -> L4pen1R {
        L4pen1R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - L4SPM1"]
    #[inline(always)]
    pub fn l4spm1(&self) -> L4spm1R {
        L4spm1R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - L4SPIM1"]
    #[inline(always)]
    pub fn l4spim1(&self) -> L4spim1R {
        L4spim1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - L4DPM1"]
    #[inline(always)]
    pub fn l4dpm1(&self) -> L4dpm1R {
        L4dpm1R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - L4DPIM1"]
    #[inline(always)]
    pub fn l4dpim1(&self) -> L4dpim1R {
        L4dpim1R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - L3PEN1"]
    #[inline(always)]
    pub fn l3pen1(&mut self) -> L3pen1W<Macl3l4c1rSpec> {
        L3pen1W::new(self, 0)
    }
    #[doc = "Bit 2 - L3SAM1"]
    #[inline(always)]
    pub fn l3sam1(&mut self) -> L3sam1W<Macl3l4c1rSpec> {
        L3sam1W::new(self, 2)
    }
    #[doc = "Bit 3 - L3SAIM1"]
    #[inline(always)]
    pub fn l3saim1(&mut self) -> L3saim1W<Macl3l4c1rSpec> {
        L3saim1W::new(self, 3)
    }
    #[doc = "Bit 4 - L3DAM1"]
    #[inline(always)]
    pub fn l3dam1(&mut self) -> L3dam1W<Macl3l4c1rSpec> {
        L3dam1W::new(self, 4)
    }
    #[doc = "Bit 5 - L3DAIM1"]
    #[inline(always)]
    pub fn l3daim1(&mut self) -> L3daim1W<Macl3l4c1rSpec> {
        L3daim1W::new(self, 5)
    }
    #[doc = "Bits 6:10 - L3HSBM1"]
    #[inline(always)]
    pub fn l3hsbm1(&mut self) -> L3hsbm1W<Macl3l4c1rSpec> {
        L3hsbm1W::new(self, 6)
    }
    #[doc = "Bits 11:15 - L3HDBM1"]
    #[inline(always)]
    pub fn l3hdbm1(&mut self) -> L3hdbm1W<Macl3l4c1rSpec> {
        L3hdbm1W::new(self, 11)
    }
    #[doc = "Bit 16 - L4PEN1"]
    #[inline(always)]
    pub fn l4pen1(&mut self) -> L4pen1W<Macl3l4c1rSpec> {
        L4pen1W::new(self, 16)
    }
    #[doc = "Bit 18 - L4SPM1"]
    #[inline(always)]
    pub fn l4spm1(&mut self) -> L4spm1W<Macl3l4c1rSpec> {
        L4spm1W::new(self, 18)
    }
    #[doc = "Bit 19 - L4SPIM1"]
    #[inline(always)]
    pub fn l4spim1(&mut self) -> L4spim1W<Macl3l4c1rSpec> {
        L4spim1W::new(self, 19)
    }
    #[doc = "Bit 20 - L4DPM1"]
    #[inline(always)]
    pub fn l4dpm1(&mut self) -> L4dpm1W<Macl3l4c1rSpec> {
        L4dpm1W::new(self, 20)
    }
    #[doc = "Bit 21 - L4DPIM1"]
    #[inline(always)]
    pub fn l4dpim1(&mut self) -> L4dpim1W<Macl3l4c1rSpec> {
        L4dpim1W::new(self, 21)
    }
}
#[doc = "L3 and L4 control 1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`macl3l4c1r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macl3l4c1r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Macl3l4c1rSpec;
impl crate::RegisterSpec for Macl3l4c1rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macl3l4c1r::R`](R) reader structure"]
impl crate::Readable for Macl3l4c1rSpec {}
#[doc = "`write(|w| ..)` method takes [`macl3l4c1r::W`](W) writer structure"]
impl crate::Writable for Macl3l4c1rSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MACL3L4C1R to value 0"]
impl crate::Resettable for Macl3l4c1rSpec {}
