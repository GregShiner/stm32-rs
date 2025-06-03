#[doc = "Register `MACL3L4C0R` reader"]
pub type R = crate::R<Macl3l4c0rSpec>;
#[doc = "Register `MACL3L4C0R` writer"]
pub type W = crate::W<Macl3l4c0rSpec>;
#[doc = "Field `L3PEN0` reader - L3PEN0"]
pub type L3pen0R = crate::BitReader;
#[doc = "Field `L3PEN0` writer - L3PEN0"]
pub type L3pen0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L3SAM0` reader - L3SAM0"]
pub type L3sam0R = crate::BitReader;
#[doc = "Field `L3SAM0` writer - L3SAM0"]
pub type L3sam0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L3SAIM0` reader - L3SAIM0"]
pub type L3saim0R = crate::BitReader;
#[doc = "Field `L3SAIM0` writer - L3SAIM0"]
pub type L3saim0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L3DAM0` reader - L3DAM0"]
pub type L3dam0R = crate::BitReader;
#[doc = "Field `L3DAM0` writer - L3DAM0"]
pub type L3dam0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L3DAIM0` reader - L3DAIM0"]
pub type L3daim0R = crate::BitReader;
#[doc = "Field `L3DAIM0` writer - L3DAIM0"]
pub type L3daim0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L3HSBM0` reader - L3HSBM0"]
pub type L3hsbm0R = crate::FieldReader;
#[doc = "Field `L3HSBM0` writer - L3HSBM0"]
pub type L3hsbm0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `L3HDBM0` reader - L3HDBM0"]
pub type L3hdbm0R = crate::FieldReader;
#[doc = "Field `L3HDBM0` writer - L3HDBM0"]
pub type L3hdbm0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `L4PEN0` reader - L4PEN0"]
pub type L4pen0R = crate::BitReader;
#[doc = "Field `L4PEN0` writer - L4PEN0"]
pub type L4pen0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L4SPM0` reader - L4SPM0"]
pub type L4spm0R = crate::BitReader;
#[doc = "Field `L4SPM0` writer - L4SPM0"]
pub type L4spm0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L4SPIM0` reader - L4SPIM0"]
pub type L4spim0R = crate::BitReader;
#[doc = "Field `L4SPIM0` writer - L4SPIM0"]
pub type L4spim0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L4DPM0` reader - L4DPM0"]
pub type L4dpm0R = crate::BitReader;
#[doc = "Field `L4DPM0` writer - L4DPM0"]
pub type L4dpm0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L4DPIM0` reader - L4DPIM0"]
pub type L4dpim0R = crate::BitReader;
#[doc = "Field `L4DPIM0` writer - L4DPIM0"]
pub type L4dpim0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - L3PEN0"]
    #[inline(always)]
    pub fn l3pen0(&self) -> L3pen0R {
        L3pen0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - L3SAM0"]
    #[inline(always)]
    pub fn l3sam0(&self) -> L3sam0R {
        L3sam0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - L3SAIM0"]
    #[inline(always)]
    pub fn l3saim0(&self) -> L3saim0R {
        L3saim0R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - L3DAM0"]
    #[inline(always)]
    pub fn l3dam0(&self) -> L3dam0R {
        L3dam0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - L3DAIM0"]
    #[inline(always)]
    pub fn l3daim0(&self) -> L3daim0R {
        L3daim0R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:10 - L3HSBM0"]
    #[inline(always)]
    pub fn l3hsbm0(&self) -> L3hsbm0R {
        L3hsbm0R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bits 11:15 - L3HDBM0"]
    #[inline(always)]
    pub fn l3hdbm0(&self) -> L3hdbm0R {
        L3hdbm0R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bit 16 - L4PEN0"]
    #[inline(always)]
    pub fn l4pen0(&self) -> L4pen0R {
        L4pen0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - L4SPM0"]
    #[inline(always)]
    pub fn l4spm0(&self) -> L4spm0R {
        L4spm0R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - L4SPIM0"]
    #[inline(always)]
    pub fn l4spim0(&self) -> L4spim0R {
        L4spim0R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - L4DPM0"]
    #[inline(always)]
    pub fn l4dpm0(&self) -> L4dpm0R {
        L4dpm0R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - L4DPIM0"]
    #[inline(always)]
    pub fn l4dpim0(&self) -> L4dpim0R {
        L4dpim0R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - L3PEN0"]
    #[inline(always)]
    pub fn l3pen0(&mut self) -> L3pen0W<Macl3l4c0rSpec> {
        L3pen0W::new(self, 0)
    }
    #[doc = "Bit 2 - L3SAM0"]
    #[inline(always)]
    pub fn l3sam0(&mut self) -> L3sam0W<Macl3l4c0rSpec> {
        L3sam0W::new(self, 2)
    }
    #[doc = "Bit 3 - L3SAIM0"]
    #[inline(always)]
    pub fn l3saim0(&mut self) -> L3saim0W<Macl3l4c0rSpec> {
        L3saim0W::new(self, 3)
    }
    #[doc = "Bit 4 - L3DAM0"]
    #[inline(always)]
    pub fn l3dam0(&mut self) -> L3dam0W<Macl3l4c0rSpec> {
        L3dam0W::new(self, 4)
    }
    #[doc = "Bit 5 - L3DAIM0"]
    #[inline(always)]
    pub fn l3daim0(&mut self) -> L3daim0W<Macl3l4c0rSpec> {
        L3daim0W::new(self, 5)
    }
    #[doc = "Bits 6:10 - L3HSBM0"]
    #[inline(always)]
    pub fn l3hsbm0(&mut self) -> L3hsbm0W<Macl3l4c0rSpec> {
        L3hsbm0W::new(self, 6)
    }
    #[doc = "Bits 11:15 - L3HDBM0"]
    #[inline(always)]
    pub fn l3hdbm0(&mut self) -> L3hdbm0W<Macl3l4c0rSpec> {
        L3hdbm0W::new(self, 11)
    }
    #[doc = "Bit 16 - L4PEN0"]
    #[inline(always)]
    pub fn l4pen0(&mut self) -> L4pen0W<Macl3l4c0rSpec> {
        L4pen0W::new(self, 16)
    }
    #[doc = "Bit 18 - L4SPM0"]
    #[inline(always)]
    pub fn l4spm0(&mut self) -> L4spm0W<Macl3l4c0rSpec> {
        L4spm0W::new(self, 18)
    }
    #[doc = "Bit 19 - L4SPIM0"]
    #[inline(always)]
    pub fn l4spim0(&mut self) -> L4spim0W<Macl3l4c0rSpec> {
        L4spim0W::new(self, 19)
    }
    #[doc = "Bit 20 - L4DPM0"]
    #[inline(always)]
    pub fn l4dpm0(&mut self) -> L4dpm0W<Macl3l4c0rSpec> {
        L4dpm0W::new(self, 20)
    }
    #[doc = "Bit 21 - L4DPIM0"]
    #[inline(always)]
    pub fn l4dpim0(&mut self) -> L4dpim0W<Macl3l4c0rSpec> {
        L4dpim0W::new(self, 21)
    }
}
#[doc = "L3 and L4 control 0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`macl3l4c0r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macl3l4c0r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Macl3l4c0rSpec;
impl crate::RegisterSpec for Macl3l4c0rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macl3l4c0r::R`](R) reader structure"]
impl crate::Readable for Macl3l4c0rSpec {}
#[doc = "`write(|w| ..)` method takes [`macl3l4c0r::W`](W) writer structure"]
impl crate::Writable for Macl3l4c0rSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MACL3L4C0R to value 0"]
impl crate::Resettable for Macl3l4c0rSpec {}
