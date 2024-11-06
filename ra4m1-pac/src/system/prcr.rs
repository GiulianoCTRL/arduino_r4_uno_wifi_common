#[doc = "Register `PRCR` reader"]
pub type R = crate::R<PRCR_SPEC>;
#[doc = "Register `PRCR` writer"]
pub type W = crate::W<PRCR_SPEC>;
#[doc = "Field `PRC0` reader - Protect Bit 0"]
pub type PRC0_R = crate::BitReader<PRC0_A>;
#[doc = "Protect Bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRC0_A {
    #[doc = "0: Writes protected."]
    _0 = 0,
    #[doc = "1: Writes not protected."]
    _1 = 1,
}
impl From<PRC0_A> for bool {
    #[inline(always)]
    fn from(variant: PRC0_A) -> Self {
        variant as u8 != 0
    }
}
impl PRC0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PRC0_A {
        match self.bits {
            false => PRC0_A::_0,
            true => PRC0_A::_1,
        }
    }
    #[doc = "Writes protected."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PRC0_A::_0
    }
    #[doc = "Writes not protected."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PRC0_A::_1
    }
}
#[doc = "Field `PRC0` writer - Protect Bit 0"]
pub type PRC0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PRC0_A>;
impl<'a, REG, const O: u8> PRC0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writes protected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PRC0_A::_0)
    }
    #[doc = "Writes not protected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PRC0_A::_1)
    }
}
#[doc = "Field `PRC1` reader - Protect Bit 1"]
pub type PRC1_R = crate::BitReader<PRC1_A>;
#[doc = "Protect Bit 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRC1_A {
    #[doc = "0: Writes protected."]
    _0 = 0,
    #[doc = "1: Writes not protected."]
    _1 = 1,
}
impl From<PRC1_A> for bool {
    #[inline(always)]
    fn from(variant: PRC1_A) -> Self {
        variant as u8 != 0
    }
}
impl PRC1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PRC1_A {
        match self.bits {
            false => PRC1_A::_0,
            true => PRC1_A::_1,
        }
    }
    #[doc = "Writes protected."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PRC1_A::_0
    }
    #[doc = "Writes not protected."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PRC1_A::_1
    }
}
#[doc = "Field `PRC1` writer - Protect Bit 1"]
pub type PRC1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PRC1_A>;
impl<'a, REG, const O: u8> PRC1_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writes protected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PRC1_A::_0)
    }
    #[doc = "Writes not protected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PRC1_A::_1)
    }
}
#[doc = "Field `PRC3` reader - Protect Bit 3"]
pub type PRC3_R = crate::BitReader<PRC3_A>;
#[doc = "Protect Bit 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRC3_A {
    #[doc = "0: Writes protected."]
    _0 = 0,
    #[doc = "1: Writes not protected."]
    _1 = 1,
}
impl From<PRC3_A> for bool {
    #[inline(always)]
    fn from(variant: PRC3_A) -> Self {
        variant as u8 != 0
    }
}
impl PRC3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PRC3_A {
        match self.bits {
            false => PRC3_A::_0,
            true => PRC3_A::_1,
        }
    }
    #[doc = "Writes protected."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PRC3_A::_0
    }
    #[doc = "Writes not protected."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PRC3_A::_1
    }
}
#[doc = "Field `PRC3` writer - Protect Bit 3"]
pub type PRC3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PRC3_A>;
impl<'a, REG, const O: u8> PRC3_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writes protected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PRC3_A::_0)
    }
    #[doc = "Writes not protected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PRC3_A::_1)
    }
}
#[doc = "PRC Key Code\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRKEY_AW {
    #[doc = "90: Enables writing to the PRCR register."]
    _0X5A = 90,
}
impl From<PRKEY_AW> for u8 {
    #[inline(always)]
    fn from(variant: PRKEY_AW) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PRKEY_AW {
    type Ux = u8;
}
#[doc = "Field `PRKEY` writer - PRC Key Code"]
pub type PRKEY_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O, PRKEY_AW>;
impl<'a, REG, const O: u8> PRKEY_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Enables writing to the PRCR register."]
    #[inline(always)]
    pub fn _0x5a(self) -> &'a mut crate::W<REG> {
        self.variant(PRKEY_AW::_0X5A)
    }
}
impl R {
    #[doc = "Bit 0 - Protect Bit 0"]
    #[inline(always)]
    pub fn prc0(&self) -> PRC0_R {
        PRC0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Protect Bit 1"]
    #[inline(always)]
    pub fn prc1(&self) -> PRC1_R {
        PRC1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Protect Bit 3"]
    #[inline(always)]
    pub fn prc3(&self) -> PRC3_R {
        PRC3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Protect Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn prc0(&mut self) -> PRC0_W<PRCR_SPEC, 0> {
        PRC0_W::new(self)
    }
    #[doc = "Bit 1 - Protect Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn prc1(&mut self) -> PRC1_W<PRCR_SPEC, 1> {
        PRC1_W::new(self)
    }
    #[doc = "Bit 3 - Protect Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn prc3(&mut self) -> PRC3_W<PRCR_SPEC, 3> {
        PRC3_W::new(self)
    }
    #[doc = "Bits 8:15 - PRC Key Code"]
    #[inline(always)]
    #[must_use]
    pub fn prkey(&mut self) -> PRKEY_W<PRCR_SPEC, 8> {
        PRKEY_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Protect Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRCR_SPEC;
impl crate::RegisterSpec for PRCR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`prcr::R`](R) reader structure"]
impl crate::Readable for PRCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`prcr::W`](W) writer structure"]
impl crate::Writable for PRCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRCR to value 0"]
impl crate::Resettable for PRCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
