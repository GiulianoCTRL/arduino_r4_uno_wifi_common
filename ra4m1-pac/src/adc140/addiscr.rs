#[doc = "Register `ADDISCR` reader"]
pub type R = crate::R<ADDISCR_SPEC>;
#[doc = "Register `ADDISCR` writer"]
pub type W = crate::W<ADDISCR_SPEC>;
#[doc = "Field `ADNDIS` reader - The charging time"]
pub type ADNDIS_R = crate::FieldReader<ADNDIS_A>;
#[doc = "The charging time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADNDIS_A {
    #[doc = "0: Disconnection detection is disabled"]
    _0000 = 0,
    #[doc = "1: Setting prohibited"]
    _0001 = 1,
}
impl From<ADNDIS_A> for u8 {
    #[inline(always)]
    fn from(variant: ADNDIS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ADNDIS_A {
    type Ux = u8;
}
impl ADNDIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ADNDIS_A> {
        match self.bits {
            0 => Some(ADNDIS_A::_0000),
            1 => Some(ADNDIS_A::_0001),
            _ => None,
        }
    }
    #[doc = "Disconnection detection is disabled"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == ADNDIS_A::_0000
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == ADNDIS_A::_0001
    }
}
#[doc = "Field `ADNDIS` writer - The charging time"]
pub type ADNDIS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O, ADNDIS_A>;
impl<'a, REG, const O: u8> ADNDIS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disconnection detection is disabled"]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut crate::W<REG> {
        self.variant(ADNDIS_A::_0000)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _0001(self) -> &'a mut crate::W<REG> {
        self.variant(ADNDIS_A::_0001)
    }
}
#[doc = "Field `PCHG` reader - Selection of Precharge or Discharge"]
pub type PCHG_R = crate::BitReader<PCHG_A>;
#[doc = "Selection of Precharge or Discharge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PCHG_A {
    #[doc = "0: Discharge"]
    _0 = 0,
    #[doc = "1: Precharge"]
    _1 = 1,
}
impl From<PCHG_A> for bool {
    #[inline(always)]
    fn from(variant: PCHG_A) -> Self {
        variant as u8 != 0
    }
}
impl PCHG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PCHG_A {
        match self.bits {
            false => PCHG_A::_0,
            true => PCHG_A::_1,
        }
    }
    #[doc = "Discharge"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PCHG_A::_0
    }
    #[doc = "Precharge"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PCHG_A::_1
    }
}
#[doc = "Field `PCHG` writer - Selection of Precharge or Discharge"]
pub type PCHG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PCHG_A>;
impl<'a, REG, const O: u8> PCHG_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Discharge"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PCHG_A::_0)
    }
    #[doc = "Precharge"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PCHG_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:3 - The charging time"]
    #[inline(always)]
    pub fn adndis(&self) -> ADNDIS_R {
        ADNDIS_R::new(self.bits & 0x0f)
    }
    #[doc = "Bit 4 - Selection of Precharge or Discharge"]
    #[inline(always)]
    pub fn pchg(&self) -> PCHG_R {
        PCHG_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - The charging time"]
    #[inline(always)]
    #[must_use]
    pub fn adndis(&mut self) -> ADNDIS_W<ADDISCR_SPEC, 0> {
        ADNDIS_W::new(self)
    }
    #[doc = "Bit 4 - Selection of Precharge or Discharge"]
    #[inline(always)]
    #[must_use]
    pub fn pchg(&mut self) -> PCHG_W<ADDISCR_SPEC, 4> {
        PCHG_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "A/D Disconnection Detection Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addiscr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addiscr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADDISCR_SPEC;
impl crate::RegisterSpec for ADDISCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`addiscr::R`](R) reader structure"]
impl crate::Readable for ADDISCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`addiscr::W`](W) writer structure"]
impl crate::Writable for ADDISCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADDISCR to value 0"]
impl crate::Resettable for ADDISCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
