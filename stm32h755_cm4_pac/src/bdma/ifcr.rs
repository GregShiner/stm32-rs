#[doc = "Register `IFCR` writer"]
pub type W = crate::W<IfcrSpec>;
#[doc = "Field `CGIF1` writer - Channel x global interrupt clear This bit is set and cleared by software."]
pub type Cgif1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTCIF1` writer - Channel x transfer complete clear This bit is set and cleared by software."]
pub type Ctcif1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHTIF1` writer - Channel x half transfer clear This bit is set and cleared by software."]
pub type Chtif1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTEIF1` writer - Channel x transfer error clear This bit is set and cleared by software."]
pub type Cteif1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CGIF2` writer - Channel x global interrupt clear This bit is set and cleared by software."]
pub type Cgif2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTCIF2` writer - Channel x transfer complete clear This bit is set and cleared by software."]
pub type Ctcif2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHTIF2` writer - Channel x half transfer clear This bit is set and cleared by software."]
pub type Chtif2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTEIF2` writer - Channel x transfer error clear This bit is set and cleared by software."]
pub type Cteif2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CGIF3` writer - Channel x global interrupt clear This bit is set and cleared by software."]
pub type Cgif3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTCIF3` writer - Channel x transfer complete clear This bit is set and cleared by software."]
pub type Ctcif3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHTIF3` writer - Channel x half transfer clear This bit is set and cleared by software."]
pub type Chtif3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTEIF3` writer - Channel x transfer error clear This bit is set and cleared by software."]
pub type Cteif3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CGIF4` writer - Channel x global interrupt clear This bit is set and cleared by software."]
pub type Cgif4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTCIF4` writer - Channel x transfer complete clear This bit is set and cleared by software."]
pub type Ctcif4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHTIF4` writer - Channel x half transfer clear This bit is set and cleared by software."]
pub type Chtif4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTEIF4` writer - Channel x transfer error clear This bit is set and cleared by software."]
pub type Cteif4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CGIF5` writer - Channel x global interrupt clear This bit is set and cleared by software."]
pub type Cgif5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTCIF5` writer - Channel x transfer complete clear This bit is set and cleared by software."]
pub type Ctcif5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHTIF5` writer - Channel x half transfer clear This bit is set and cleared by software."]
pub type Chtif5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTEIF5` writer - Channel x transfer error clear This bit is set and cleared by software."]
pub type Cteif5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CGIF6` writer - Channel x global interrupt clear This bit is set and cleared by software."]
pub type Cgif6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTCIF6` writer - Channel x transfer complete clear This bit is set and cleared by software."]
pub type Ctcif6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHTIF6` writer - Channel x half transfer clear This bit is set and cleared by software."]
pub type Chtif6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTEIF6` writer - Channel x transfer error clear This bit is set and cleared by software."]
pub type Cteif6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CGIF7` writer - Channel x global interrupt clear This bit is set and cleared by software."]
pub type Cgif7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTCIF7` writer - Channel x transfer complete clear This bit is set and cleared by software."]
pub type Ctcif7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHTIF7` writer - Channel x half transfer clear This bit is set and cleared by software."]
pub type Chtif7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTEIF7` writer - Channel x transfer error clear This bit is set and cleared by software."]
pub type Cteif7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CGIF8` writer - Channel x global interrupt clear This bit is set and cleared by software."]
pub type Cgif8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTCIF8` writer - Channel x transfer complete clear This bit is set and cleared by software."]
pub type Ctcif8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHTIF8` writer - Channel x half transfer clear This bit is set and cleared by software."]
pub type Chtif8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTEIF8` writer - Channel x transfer error clear This bit is set and cleared by software."]
pub type Cteif8W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Channel x global interrupt clear This bit is set and cleared by software."]
    #[inline(always)]
    pub fn cgif1(&mut self) -> Cgif1W<IfcrSpec> {
        Cgif1W::new(self, 0)
    }
    #[doc = "Bit 1 - Channel x transfer complete clear This bit is set and cleared by software."]
    #[inline(always)]
    pub fn ctcif1(&mut self) -> Ctcif1W<IfcrSpec> {
        Ctcif1W::new(self, 1)
    }
    #[doc = "Bit 2 - Channel x half transfer clear This bit is set and cleared by software."]
    #[inline(always)]
    pub fn chtif1(&mut self) -> Chtif1W<IfcrSpec> {
        Chtif1W::new(self, 2)
    }
    #[doc = "Bit 3 - Channel x transfer error clear This bit is set and cleared by software."]
    #[inline(always)]
    pub fn cteif1(&mut self) -> Cteif1W<IfcrSpec> {
        Cteif1W::new(self, 3)
    }
    #[doc = "Bit 4 - Channel x global interrupt clear This bit is set and cleared by software."]
    #[inline(always)]
    pub fn cgif2(&mut self) -> Cgif2W<IfcrSpec> {
        Cgif2W::new(self, 4)
    }
    #[doc = "Bit 5 - Channel x transfer complete clear This bit is set and cleared by software."]
    #[inline(always)]
    pub fn ctcif2(&mut self) -> Ctcif2W<IfcrSpec> {
        Ctcif2W::new(self, 5)
    }
    #[doc = "Bit 6 - Channel x half transfer clear This bit is set and cleared by software."]
    #[inline(always)]
    pub fn chtif2(&mut self) -> Chtif2W<IfcrSpec> {
        Chtif2W::new(self, 6)
    }
    #[doc = "Bit 7 - Channel x transfer error clear This bit is set and cleared by software."]
    #[inline(always)]
    pub fn cteif2(&mut self) -> Cteif2W<IfcrSpec> {
        Cteif2W::new(self, 7)
    }
    #[doc = "Bit 8 - Channel x global interrupt clear This bit is set and cleared by software."]
    #[inline(always)]
    pub fn cgif3(&mut self) -> Cgif3W<IfcrSpec> {
        Cgif3W::new(self, 8)
    }
    #[doc = "Bit 9 - Channel x transfer complete clear This bit is set and cleared by software."]
    #[inline(always)]
    pub fn ctcif3(&mut self) -> Ctcif3W<IfcrSpec> {
        Ctcif3W::new(self, 9)
    }
    #[doc = "Bit 10 - Channel x half transfer clear This bit is set and cleared by software."]
    #[inline(always)]
    pub fn chtif3(&mut self) -> Chtif3W<IfcrSpec> {
        Chtif3W::new(self, 10)
    }
    #[doc = "Bit 11 - Channel x transfer error clear This bit is set and cleared by software."]
    #[inline(always)]
    pub fn cteif3(&mut self) -> Cteif3W<IfcrSpec> {
        Cteif3W::new(self, 11)
    }
    #[doc = "Bit 12 - Channel x global interrupt clear This bit is set and cleared by software."]
    #[inline(always)]
    pub fn cgif4(&mut self) -> Cgif4W<IfcrSpec> {
        Cgif4W::new(self, 12)
    }
    #[doc = "Bit 13 - Channel x transfer complete clear This bit is set and cleared by software."]
    #[inline(always)]
    pub fn ctcif4(&mut self) -> Ctcif4W<IfcrSpec> {
        Ctcif4W::new(self, 13)
    }
    #[doc = "Bit 14 - Channel x half transfer clear This bit is set and cleared by software."]
    #[inline(always)]
    pub fn chtif4(&mut self) -> Chtif4W<IfcrSpec> {
        Chtif4W::new(self, 14)
    }
    #[doc = "Bit 15 - Channel x transfer error clear This bit is set and cleared by software."]
    #[inline(always)]
    pub fn cteif4(&mut self) -> Cteif4W<IfcrSpec> {
        Cteif4W::new(self, 15)
    }
    #[doc = "Bit 16 - Channel x global interrupt clear This bit is set and cleared by software."]
    #[inline(always)]
    pub fn cgif5(&mut self) -> Cgif5W<IfcrSpec> {
        Cgif5W::new(self, 16)
    }
    #[doc = "Bit 17 - Channel x transfer complete clear This bit is set and cleared by software."]
    #[inline(always)]
    pub fn ctcif5(&mut self) -> Ctcif5W<IfcrSpec> {
        Ctcif5W::new(self, 17)
    }
    #[doc = "Bit 18 - Channel x half transfer clear This bit is set and cleared by software."]
    #[inline(always)]
    pub fn chtif5(&mut self) -> Chtif5W<IfcrSpec> {
        Chtif5W::new(self, 18)
    }
    #[doc = "Bit 19 - Channel x transfer error clear This bit is set and cleared by software."]
    #[inline(always)]
    pub fn cteif5(&mut self) -> Cteif5W<IfcrSpec> {
        Cteif5W::new(self, 19)
    }
    #[doc = "Bit 20 - Channel x global interrupt clear This bit is set and cleared by software."]
    #[inline(always)]
    pub fn cgif6(&mut self) -> Cgif6W<IfcrSpec> {
        Cgif6W::new(self, 20)
    }
    #[doc = "Bit 21 - Channel x transfer complete clear This bit is set and cleared by software."]
    #[inline(always)]
    pub fn ctcif6(&mut self) -> Ctcif6W<IfcrSpec> {
        Ctcif6W::new(self, 21)
    }
    #[doc = "Bit 22 - Channel x half transfer clear This bit is set and cleared by software."]
    #[inline(always)]
    pub fn chtif6(&mut self) -> Chtif6W<IfcrSpec> {
        Chtif6W::new(self, 22)
    }
    #[doc = "Bit 23 - Channel x transfer error clear This bit is set and cleared by software."]
    #[inline(always)]
    pub fn cteif6(&mut self) -> Cteif6W<IfcrSpec> {
        Cteif6W::new(self, 23)
    }
    #[doc = "Bit 24 - Channel x global interrupt clear This bit is set and cleared by software."]
    #[inline(always)]
    pub fn cgif7(&mut self) -> Cgif7W<IfcrSpec> {
        Cgif7W::new(self, 24)
    }
    #[doc = "Bit 25 - Channel x transfer complete clear This bit is set and cleared by software."]
    #[inline(always)]
    pub fn ctcif7(&mut self) -> Ctcif7W<IfcrSpec> {
        Ctcif7W::new(self, 25)
    }
    #[doc = "Bit 26 - Channel x half transfer clear This bit is set and cleared by software."]
    #[inline(always)]
    pub fn chtif7(&mut self) -> Chtif7W<IfcrSpec> {
        Chtif7W::new(self, 26)
    }
    #[doc = "Bit 27 - Channel x transfer error clear This bit is set and cleared by software."]
    #[inline(always)]
    pub fn cteif7(&mut self) -> Cteif7W<IfcrSpec> {
        Cteif7W::new(self, 27)
    }
    #[doc = "Bit 28 - Channel x global interrupt clear This bit is set and cleared by software."]
    #[inline(always)]
    pub fn cgif8(&mut self) -> Cgif8W<IfcrSpec> {
        Cgif8W::new(self, 28)
    }
    #[doc = "Bit 29 - Channel x transfer complete clear This bit is set and cleared by software."]
    #[inline(always)]
    pub fn ctcif8(&mut self) -> Ctcif8W<IfcrSpec> {
        Ctcif8W::new(self, 29)
    }
    #[doc = "Bit 30 - Channel x half transfer clear This bit is set and cleared by software."]
    #[inline(always)]
    pub fn chtif8(&mut self) -> Chtif8W<IfcrSpec> {
        Chtif8W::new(self, 30)
    }
    #[doc = "Bit 31 - Channel x transfer error clear This bit is set and cleared by software."]
    #[inline(always)]
    pub fn cteif8(&mut self) -> Cteif8W<IfcrSpec> {
        Cteif8W::new(self, 31)
    }
}
#[doc = "DMA interrupt flag clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IfcrSpec;
impl crate::RegisterSpec for IfcrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ifcr::W`](W) writer structure"]
impl crate::Writable for IfcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IFCR to value 0"]
impl crate::Resettable for IfcrSpec {}
