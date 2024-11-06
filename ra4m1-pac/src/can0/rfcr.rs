#[doc = "Register `RFCR` reader"]
pub type R = crate::R<RFCR_SPEC>;
#[doc = "Register `RFCR` writer"]
pub type W = crate::W<RFCR_SPEC>;
#[doc = "Field `RFE` reader - Receive FIFO Enable"]
pub type RFE_R = crate::BitReader<RFE_A>;
#[doc = "Receive FIFO Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFE_A {
    #[doc = "0: Receive FIFO disabled"]
    _0 = 0,
    #[doc = "1: Receive FIFO enabled"]
    _1 = 1,
}
impl From<RFE_A> for bool {
    #[inline(always)]
    fn from(variant: RFE_A) -> Self {
        variant as u8 != 0
    }
}
impl RFE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RFE_A {
        match self.bits {
            false => RFE_A::_0,
            true => RFE_A::_1,
        }
    }
    #[doc = "Receive FIFO disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RFE_A::_0
    }
    #[doc = "Receive FIFO enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RFE_A::_1
    }
}
#[doc = "Field `RFE` writer - Receive FIFO Enable"]
pub type RFE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RFE_A>;
impl<'a, REG, const O: u8> RFE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Receive FIFO disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RFE_A::_0)
    }
    #[doc = "Receive FIFO enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RFE_A::_1)
    }
}
#[doc = "Field `RFUST` reader - Receive FIFO Unread Message Number Status"]
pub type RFUST_R = crate::FieldReader<RFUST_A>;
#[doc = "Receive FIFO Unread Message Number Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RFUST_A {
    #[doc = "0: No unread message"]
    _000 = 0,
    #[doc = "1: 1 unread message"]
    _001 = 1,
    #[doc = "2: 2 unread messages"]
    _010 = 2,
    #[doc = "3: 3 unread messages"]
    _011 = 3,
    #[doc = "4: 4 unread messages"]
    _100 = 4,
}
impl From<RFUST_A> for u8 {
    #[inline(always)]
    fn from(variant: RFUST_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RFUST_A {
    type Ux = u8;
}
impl RFUST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<RFUST_A> {
        match self.bits {
            0 => Some(RFUST_A::_000),
            1 => Some(RFUST_A::_001),
            2 => Some(RFUST_A::_010),
            3 => Some(RFUST_A::_011),
            4 => Some(RFUST_A::_100),
            _ => None,
        }
    }
    #[doc = "No unread message"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == RFUST_A::_000
    }
    #[doc = "1 unread message"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == RFUST_A::_001
    }
    #[doc = "2 unread messages"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == RFUST_A::_010
    }
    #[doc = "3 unread messages"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == RFUST_A::_011
    }
    #[doc = "4 unread messages"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == RFUST_A::_100
    }
}
#[doc = "Field `RFMLF` reader - Receive FIFO Message Lost Flag"]
pub type RFMLF_R = crate::BitReader<RFMLF_A>;
#[doc = "Receive FIFO Message Lost Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFMLF_A {
    #[doc = "0: No receive FIFO message lost has occurred"]
    _0 = 0,
    #[doc = "1: Receive FIFO message lost has occurred"]
    _1 = 1,
}
impl From<RFMLF_A> for bool {
    #[inline(always)]
    fn from(variant: RFMLF_A) -> Self {
        variant as u8 != 0
    }
}
impl RFMLF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RFMLF_A {
        match self.bits {
            false => RFMLF_A::_0,
            true => RFMLF_A::_1,
        }
    }
    #[doc = "No receive FIFO message lost has occurred"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RFMLF_A::_0
    }
    #[doc = "Receive FIFO message lost has occurred"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RFMLF_A::_1
    }
}
#[doc = "Field `RFMLF` writer - Receive FIFO Message Lost Flag"]
pub type RFMLF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RFMLF_A>;
impl<'a, REG, const O: u8> RFMLF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No receive FIFO message lost has occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RFMLF_A::_0)
    }
    #[doc = "Receive FIFO message lost has occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RFMLF_A::_1)
    }
}
#[doc = "Field `RFFST` reader - Receive FIFO Full Status Flag"]
pub type RFFST_R = crate::BitReader<RFFST_A>;
#[doc = "Receive FIFO Full Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFFST_A {
    #[doc = "0: Receive FIFO is not full"]
    _0 = 0,
    #[doc = "1: Receive FIFO is full (4 unread messages)"]
    _1 = 1,
}
impl From<RFFST_A> for bool {
    #[inline(always)]
    fn from(variant: RFFST_A) -> Self {
        variant as u8 != 0
    }
}
impl RFFST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RFFST_A {
        match self.bits {
            false => RFFST_A::_0,
            true => RFFST_A::_1,
        }
    }
    #[doc = "Receive FIFO is not full"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RFFST_A::_0
    }
    #[doc = "Receive FIFO is full (4 unread messages)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RFFST_A::_1
    }
}
#[doc = "Field `RFWST` reader - Receive FIFO Buffer Warning Status Flag"]
pub type RFWST_R = crate::BitReader<RFWST_A>;
#[doc = "Receive FIFO Buffer Warning Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFWST_A {
    #[doc = "0: Receive FIFO is not buffer warning"]
    _0 = 0,
    #[doc = "1: Receive FIFO is buffer warning (3 unread messages)"]
    _1 = 1,
}
impl From<RFWST_A> for bool {
    #[inline(always)]
    fn from(variant: RFWST_A) -> Self {
        variant as u8 != 0
    }
}
impl RFWST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RFWST_A {
        match self.bits {
            false => RFWST_A::_0,
            true => RFWST_A::_1,
        }
    }
    #[doc = "Receive FIFO is not buffer warning"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RFWST_A::_0
    }
    #[doc = "Receive FIFO is buffer warning (3 unread messages)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RFWST_A::_1
    }
}
#[doc = "Field `RFEST` reader - Receive FIFO Empty Status Flag"]
pub type RFEST_R = crate::BitReader<RFEST_A>;
#[doc = "Receive FIFO Empty Status Flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFEST_A {
    #[doc = "0: Unread message in receive FIFO"]
    _0 = 0,
    #[doc = "1: No unread message in receive FIFO"]
    _1 = 1,
}
impl From<RFEST_A> for bool {
    #[inline(always)]
    fn from(variant: RFEST_A) -> Self {
        variant as u8 != 0
    }
}
impl RFEST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RFEST_A {
        match self.bits {
            false => RFEST_A::_0,
            true => RFEST_A::_1,
        }
    }
    #[doc = "Unread message in receive FIFO"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RFEST_A::_0
    }
    #[doc = "No unread message in receive FIFO"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RFEST_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - Receive FIFO Enable"]
    #[inline(always)]
    pub fn rfe(&self) -> RFE_R {
        RFE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - Receive FIFO Unread Message Number Status"]
    #[inline(always)]
    pub fn rfust(&self) -> RFUST_R {
        RFUST_R::new((self.bits >> 1) & 7)
    }
    #[doc = "Bit 4 - Receive FIFO Message Lost Flag"]
    #[inline(always)]
    pub fn rfmlf(&self) -> RFMLF_R {
        RFMLF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Receive FIFO Full Status Flag"]
    #[inline(always)]
    pub fn rffst(&self) -> RFFST_R {
        RFFST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive FIFO Buffer Warning Status Flag"]
    #[inline(always)]
    pub fn rfwst(&self) -> RFWST_R {
        RFWST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Receive FIFO Empty Status Flag"]
    #[inline(always)]
    pub fn rfest(&self) -> RFEST_R {
        RFEST_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receive FIFO Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rfe(&mut self) -> RFE_W<RFCR_SPEC, 0> {
        RFE_W::new(self)
    }
    #[doc = "Bit 4 - Receive FIFO Message Lost Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rfmlf(&mut self) -> RFMLF_W<RFCR_SPEC, 4> {
        RFMLF_W::new(self)
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
#[doc = "Receive FIFO Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rfcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RFCR_SPEC;
impl crate::RegisterSpec for RFCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`rfcr::R`](R) reader structure"]
impl crate::Readable for RFCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rfcr::W`](W) writer structure"]
impl crate::Writable for RFCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RFCR to value 0x80"]
impl crate::Resettable for RFCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x80;
}
