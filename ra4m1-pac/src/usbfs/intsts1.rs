#[doc = "Register `INTSTS1` reader"]
pub type R = crate::R<INTSTS1_SPEC>;
#[doc = "Register `INTSTS1` writer"]
pub type W = crate::W<INTSTS1_SPEC>;
#[doc = "Field `PDDETINT0` reader - PDDET0 Detection Interrupt Status\n\nThe field is **modified** in some way after a read operation."]
pub type PDDETINT0_R = crate::BitReader<PDDETINT0_A>;
#[doc = "PDDET0 Detection Interrupt Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDDETINT0_A {
    #[doc = "0: PDDET0 detection interrupts are not generated."]
    _0 = 0,
    #[doc = "1: PDDET0 detection interrupts are generated."]
    _1 = 1,
}
impl From<PDDETINT0_A> for bool {
    #[inline(always)]
    fn from(variant: PDDETINT0_A) -> Self {
        variant as u8 != 0
    }
}
impl PDDETINT0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PDDETINT0_A {
        match self.bits {
            false => PDDETINT0_A::_0,
            true => PDDETINT0_A::_1,
        }
    }
    #[doc = "PDDET0 detection interrupts are not generated."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDDETINT0_A::_0
    }
    #[doc = "PDDET0 detection interrupts are generated."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDDETINT0_A::_1
    }
}
#[doc = "Field `PDDETINT0` writer - PDDET0 Detection Interrupt Status"]
pub type PDDETINT0_W<'a, REG, const O: u8> = crate::BitWriter0C<'a, REG, O, PDDETINT0_A>;
impl<'a, REG, const O: u8> PDDETINT0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PDDET0 detection interrupts are not generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PDDETINT0_A::_0)
    }
    #[doc = "PDDET0 detection interrupts are generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PDDETINT0_A::_1)
    }
}
#[doc = "Field `SACK` reader - Setup Transaction Normal Response Interrupt Status\n\nThe field is **modified** in some way after a read operation."]
pub type SACK_R = crate::BitReader<SACK_A>;
#[doc = "Setup Transaction Normal Response Interrupt Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SACK_A {
    #[doc = "0: SACK interrupts are not generated."]
    _0 = 0,
    #[doc = "1: SACK interrupts are generated."]
    _1 = 1,
}
impl From<SACK_A> for bool {
    #[inline(always)]
    fn from(variant: SACK_A) -> Self {
        variant as u8 != 0
    }
}
impl SACK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SACK_A {
        match self.bits {
            false => SACK_A::_0,
            true => SACK_A::_1,
        }
    }
    #[doc = "SACK interrupts are not generated."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SACK_A::_0
    }
    #[doc = "SACK interrupts are generated."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SACK_A::_1
    }
}
#[doc = "Field `SACK` writer - Setup Transaction Normal Response Interrupt Status"]
pub type SACK_W<'a, REG, const O: u8> = crate::BitWriter0C<'a, REG, O, SACK_A>;
impl<'a, REG, const O: u8> SACK_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SACK interrupts are not generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SACK_A::_0)
    }
    #[doc = "SACK interrupts are generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SACK_A::_1)
    }
}
#[doc = "Field `SIGN` reader - Setup Transaction Error Interrupt Status\n\nThe field is **modified** in some way after a read operation."]
pub type SIGN_R = crate::BitReader<SIGN_A>;
#[doc = "Setup Transaction Error Interrupt Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SIGN_A {
    #[doc = "0: SIGN interrupts are not generated."]
    _0 = 0,
    #[doc = "1: SIGN interrupts are generated."]
    _1 = 1,
}
impl From<SIGN_A> for bool {
    #[inline(always)]
    fn from(variant: SIGN_A) -> Self {
        variant as u8 != 0
    }
}
impl SIGN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SIGN_A {
        match self.bits {
            false => SIGN_A::_0,
            true => SIGN_A::_1,
        }
    }
    #[doc = "SIGN interrupts are not generated."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SIGN_A::_0
    }
    #[doc = "SIGN interrupts are generated."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SIGN_A::_1
    }
}
#[doc = "Field `SIGN` writer - Setup Transaction Error Interrupt Status"]
pub type SIGN_W<'a, REG, const O: u8> = crate::BitWriter0C<'a, REG, O, SIGN_A>;
impl<'a, REG, const O: u8> SIGN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SIGN interrupts are not generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SIGN_A::_0)
    }
    #[doc = "SIGN interrupts are generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SIGN_A::_1)
    }
}
#[doc = "Field `EOFERR` reader - EOF Error Detection Interrupt Status\n\nThe field is **modified** in some way after a read operation."]
pub type EOFERR_R = crate::BitReader<EOFERR_A>;
#[doc = "EOF Error Detection Interrupt Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOFERR_A {
    #[doc = "0: EOFERR interrupts are not generated."]
    _0 = 0,
    #[doc = "1: EOFERR interrupts are generated."]
    _1 = 1,
}
impl From<EOFERR_A> for bool {
    #[inline(always)]
    fn from(variant: EOFERR_A) -> Self {
        variant as u8 != 0
    }
}
impl EOFERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EOFERR_A {
        match self.bits {
            false => EOFERR_A::_0,
            true => EOFERR_A::_1,
        }
    }
    #[doc = "EOFERR interrupts are not generated."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EOFERR_A::_0
    }
    #[doc = "EOFERR interrupts are generated."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EOFERR_A::_1
    }
}
#[doc = "Field `EOFERR` writer - EOF Error Detection Interrupt Status"]
pub type EOFERR_W<'a, REG, const O: u8> = crate::BitWriter0C<'a, REG, O, EOFERR_A>;
impl<'a, REG, const O: u8> EOFERR_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "EOFERR interrupts are not generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(EOFERR_A::_0)
    }
    #[doc = "EOFERR interrupts are generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(EOFERR_A::_1)
    }
}
#[doc = "Field `ATTCH` reader - ATTCH Interrupt Status\n\nThe field is **modified** in some way after a read operation."]
pub type ATTCH_R = crate::BitReader<ATTCH_A>;
#[doc = "ATTCH Interrupt Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ATTCH_A {
    #[doc = "0: ATTCH interrupts are not generated."]
    _0 = 0,
    #[doc = "1: ATTCH interrupts are generated."]
    _1 = 1,
}
impl From<ATTCH_A> for bool {
    #[inline(always)]
    fn from(variant: ATTCH_A) -> Self {
        variant as u8 != 0
    }
}
impl ATTCH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ATTCH_A {
        match self.bits {
            false => ATTCH_A::_0,
            true => ATTCH_A::_1,
        }
    }
    #[doc = "ATTCH interrupts are not generated."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ATTCH_A::_0
    }
    #[doc = "ATTCH interrupts are generated."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ATTCH_A::_1
    }
}
#[doc = "Field `ATTCH` writer - ATTCH Interrupt Status"]
pub type ATTCH_W<'a, REG, const O: u8> = crate::BitWriter0C<'a, REG, O, ATTCH_A>;
impl<'a, REG, const O: u8> ATTCH_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ATTCH interrupts are not generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ATTCH_A::_0)
    }
    #[doc = "ATTCH interrupts are generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ATTCH_A::_1)
    }
}
#[doc = "Field `DTCH` reader - USB Disconnection Detection Interrupt Status\n\nThe field is **modified** in some way after a read operation."]
pub type DTCH_R = crate::BitReader<DTCH_A>;
#[doc = "USB Disconnection Detection Interrupt Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTCH_A {
    #[doc = "0: DTCH interrupts are not generated."]
    _0 = 0,
    #[doc = "1: DTCH interrupts are generated."]
    _1 = 1,
}
impl From<DTCH_A> for bool {
    #[inline(always)]
    fn from(variant: DTCH_A) -> Self {
        variant as u8 != 0
    }
}
impl DTCH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DTCH_A {
        match self.bits {
            false => DTCH_A::_0,
            true => DTCH_A::_1,
        }
    }
    #[doc = "DTCH interrupts are not generated."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DTCH_A::_0
    }
    #[doc = "DTCH interrupts are generated."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DTCH_A::_1
    }
}
#[doc = "Field `DTCH` writer - USB Disconnection Detection Interrupt Status"]
pub type DTCH_W<'a, REG, const O: u8> = crate::BitWriter0C<'a, REG, O, DTCH_A>;
impl<'a, REG, const O: u8> DTCH_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DTCH interrupts are not generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DTCH_A::_0)
    }
    #[doc = "DTCH interrupts are generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DTCH_A::_1)
    }
}
#[doc = "Field `BCHG` reader - USB Bus Change Interrupt Status\n\nThe field is **modified** in some way after a read operation."]
pub type BCHG_R = crate::BitReader<BCHG_A>;
#[doc = "USB Bus Change Interrupt Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BCHG_A {
    #[doc = "0: BCHG interrupts are not generated."]
    _0 = 0,
    #[doc = "1: BCHG interrupts are generated."]
    _1 = 1,
}
impl From<BCHG_A> for bool {
    #[inline(always)]
    fn from(variant: BCHG_A) -> Self {
        variant as u8 != 0
    }
}
impl BCHG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BCHG_A {
        match self.bits {
            false => BCHG_A::_0,
            true => BCHG_A::_1,
        }
    }
    #[doc = "BCHG interrupts are not generated."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BCHG_A::_0
    }
    #[doc = "BCHG interrupts are generated."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BCHG_A::_1
    }
}
#[doc = "Field `BCHG` writer - USB Bus Change Interrupt Status"]
pub type BCHG_W<'a, REG, const O: u8> = crate::BitWriter0C<'a, REG, O, BCHG_A>;
impl<'a, REG, const O: u8> BCHG_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "BCHG interrupts are not generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(BCHG_A::_0)
    }
    #[doc = "BCHG interrupts are generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(BCHG_A::_1)
    }
}
#[doc = "Field `OVRCR` reader - Overcurrent Input Change Interrupt Status\n\nThe field is **modified** in some way after a read operation."]
pub type OVRCR_R = crate::BitReader<OVRCR_A>;
#[doc = "Overcurrent Input Change Interrupt Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVRCR_A {
    #[doc = "0: OVRCR interrupts are not generated."]
    _0 = 0,
    #[doc = "1: OVRCR interrupts are generated."]
    _1 = 1,
}
impl From<OVRCR_A> for bool {
    #[inline(always)]
    fn from(variant: OVRCR_A) -> Self {
        variant as u8 != 0
    }
}
impl OVRCR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OVRCR_A {
        match self.bits {
            false => OVRCR_A::_0,
            true => OVRCR_A::_1,
        }
    }
    #[doc = "OVRCR interrupts are not generated."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OVRCR_A::_0
    }
    #[doc = "OVRCR interrupts are generated."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OVRCR_A::_1
    }
}
#[doc = "Field `OVRCR` writer - Overcurrent Input Change Interrupt Status"]
pub type OVRCR_W<'a, REG, const O: u8> = crate::BitWriter0C<'a, REG, O, OVRCR_A>;
impl<'a, REG, const O: u8> OVRCR_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "OVRCR interrupts are not generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(OVRCR_A::_0)
    }
    #[doc = "OVRCR interrupts are generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(OVRCR_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - PDDET0 Detection Interrupt Status"]
    #[inline(always)]
    pub fn pddetint0(&self) -> PDDETINT0_R {
        PDDETINT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Setup Transaction Normal Response Interrupt Status"]
    #[inline(always)]
    pub fn sack(&self) -> SACK_R {
        SACK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Setup Transaction Error Interrupt Status"]
    #[inline(always)]
    pub fn sign(&self) -> SIGN_R {
        SIGN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - EOF Error Detection Interrupt Status"]
    #[inline(always)]
    pub fn eoferr(&self) -> EOFERR_R {
        EOFERR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 11 - ATTCH Interrupt Status"]
    #[inline(always)]
    pub fn attch(&self) -> ATTCH_R {
        ATTCH_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - USB Disconnection Detection Interrupt Status"]
    #[inline(always)]
    pub fn dtch(&self) -> DTCH_R {
        DTCH_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - USB Bus Change Interrupt Status"]
    #[inline(always)]
    pub fn bchg(&self) -> BCHG_R {
        BCHG_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Overcurrent Input Change Interrupt Status"]
    #[inline(always)]
    pub fn ovrcr(&self) -> OVRCR_R {
        OVRCR_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PDDET0 Detection Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn pddetint0(&mut self) -> PDDETINT0_W<INTSTS1_SPEC, 0> {
        PDDETINT0_W::new(self)
    }
    #[doc = "Bit 4 - Setup Transaction Normal Response Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn sack(&mut self) -> SACK_W<INTSTS1_SPEC, 4> {
        SACK_W::new(self)
    }
    #[doc = "Bit 5 - Setup Transaction Error Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn sign(&mut self) -> SIGN_W<INTSTS1_SPEC, 5> {
        SIGN_W::new(self)
    }
    #[doc = "Bit 6 - EOF Error Detection Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn eoferr(&mut self) -> EOFERR_W<INTSTS1_SPEC, 6> {
        EOFERR_W::new(self)
    }
    #[doc = "Bit 11 - ATTCH Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn attch(&mut self) -> ATTCH_W<INTSTS1_SPEC, 11> {
        ATTCH_W::new(self)
    }
    #[doc = "Bit 12 - USB Disconnection Detection Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn dtch(&mut self) -> DTCH_W<INTSTS1_SPEC, 12> {
        DTCH_W::new(self)
    }
    #[doc = "Bit 14 - USB Bus Change Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn bchg(&mut self) -> BCHG_W<INTSTS1_SPEC, 14> {
        BCHG_W::new(self)
    }
    #[doc = "Bit 15 - Overcurrent Input Change Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn ovrcr(&mut self) -> OVRCR_W<INTSTS1_SPEC, 15> {
        OVRCR_W::new(self)
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
#[doc = "Interrupt Status Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intsts1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intsts1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTSTS1_SPEC;
impl crate::RegisterSpec for INTSTS1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`intsts1::R`](R) reader structure"]
impl crate::Readable for INTSTS1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intsts1::W`](W) writer structure"]
impl crate::Writable for INTSTS1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0xd871;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTSTS1 to value 0"]
impl crate::Resettable for INTSTS1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
