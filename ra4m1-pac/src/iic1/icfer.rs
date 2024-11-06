#[doc = "Register `ICFER` reader"]
pub type R = crate::R<ICFER_SPEC>;
#[doc = "Register `ICFER` writer"]
pub type W = crate::W<ICFER_SPEC>;
#[doc = "Field `TMOE` reader - Timeout Function Enable"]
pub type TMOE_R = crate::BitReader<TMOE_A>;
#[doc = "Timeout Function Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TMOE_A {
    #[doc = "0: The timeout function is disabled."]
    _0 = 0,
    #[doc = "1: The timeout function is enabled."]
    _1 = 1,
}
impl From<TMOE_A> for bool {
    #[inline(always)]
    fn from(variant: TMOE_A) -> Self {
        variant as u8 != 0
    }
}
impl TMOE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TMOE_A {
        match self.bits {
            false => TMOE_A::_0,
            true => TMOE_A::_1,
        }
    }
    #[doc = "The timeout function is disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TMOE_A::_0
    }
    #[doc = "The timeout function is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TMOE_A::_1
    }
}
#[doc = "Field `TMOE` writer - Timeout Function Enable"]
pub type TMOE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TMOE_A>;
impl<'a, REG, const O: u8> TMOE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The timeout function is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TMOE_A::_0)
    }
    #[doc = "The timeout function is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TMOE_A::_1)
    }
}
#[doc = "Field `MALE` reader - Master Arbitration-Lost Detection Enable"]
pub type MALE_R = crate::BitReader<MALE_A>;
#[doc = "Master Arbitration-Lost Detection Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MALE_A {
    #[doc = "0: Master arbitration-lost detection is disabled."]
    _0 = 0,
    #[doc = "1: Master arbitration-lost detection is enabled."]
    _1 = 1,
}
impl From<MALE_A> for bool {
    #[inline(always)]
    fn from(variant: MALE_A) -> Self {
        variant as u8 != 0
    }
}
impl MALE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MALE_A {
        match self.bits {
            false => MALE_A::_0,
            true => MALE_A::_1,
        }
    }
    #[doc = "Master arbitration-lost detection is disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MALE_A::_0
    }
    #[doc = "Master arbitration-lost detection is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MALE_A::_1
    }
}
#[doc = "Field `MALE` writer - Master Arbitration-Lost Detection Enable"]
pub type MALE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MALE_A>;
impl<'a, REG, const O: u8> MALE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Master arbitration-lost detection is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MALE_A::_0)
    }
    #[doc = "Master arbitration-lost detection is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MALE_A::_1)
    }
}
#[doc = "Field `NALE` reader - NACK Transmission Arbitration-Lost Detection Enable"]
pub type NALE_R = crate::BitReader<NALE_A>;
#[doc = "NACK Transmission Arbitration-Lost Detection Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NALE_A {
    #[doc = "0: NACK transmission arbitration-lost detection is disabled."]
    _0 = 0,
    #[doc = "1: NACK transmission arbitration-lost detection is enabled."]
    _1 = 1,
}
impl From<NALE_A> for bool {
    #[inline(always)]
    fn from(variant: NALE_A) -> Self {
        variant as u8 != 0
    }
}
impl NALE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> NALE_A {
        match self.bits {
            false => NALE_A::_0,
            true => NALE_A::_1,
        }
    }
    #[doc = "NACK transmission arbitration-lost detection is disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NALE_A::_0
    }
    #[doc = "NACK transmission arbitration-lost detection is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NALE_A::_1
    }
}
#[doc = "Field `NALE` writer - NACK Transmission Arbitration-Lost Detection Enable"]
pub type NALE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, NALE_A>;
impl<'a, REG, const O: u8> NALE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NACK transmission arbitration-lost detection is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(NALE_A::_0)
    }
    #[doc = "NACK transmission arbitration-lost detection is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(NALE_A::_1)
    }
}
#[doc = "Field `SALE` reader - Slave Arbitration-Lost Detection Enable"]
pub type SALE_R = crate::BitReader<SALE_A>;
#[doc = "Slave Arbitration-Lost Detection Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SALE_A {
    #[doc = "0: Slave arbitration-lost detection is disabled."]
    _0 = 0,
    #[doc = "1: Slave arbitration-lost detection is enabled."]
    _1 = 1,
}
impl From<SALE_A> for bool {
    #[inline(always)]
    fn from(variant: SALE_A) -> Self {
        variant as u8 != 0
    }
}
impl SALE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SALE_A {
        match self.bits {
            false => SALE_A::_0,
            true => SALE_A::_1,
        }
    }
    #[doc = "Slave arbitration-lost detection is disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SALE_A::_0
    }
    #[doc = "Slave arbitration-lost detection is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SALE_A::_1
    }
}
#[doc = "Field `SALE` writer - Slave Arbitration-Lost Detection Enable"]
pub type SALE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SALE_A>;
impl<'a, REG, const O: u8> SALE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Slave arbitration-lost detection is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SALE_A::_0)
    }
    #[doc = "Slave arbitration-lost detection is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SALE_A::_1)
    }
}
#[doc = "Field `NACKE` reader - NACK Reception Transfer Suspension Enable"]
pub type NACKE_R = crate::BitReader<NACKE_A>;
#[doc = "NACK Reception Transfer Suspension Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NACKE_A {
    #[doc = "0: Transfer operation is not suspended during NACK reception (transfer suspension disabled)."]
    _0 = 0,
    #[doc = "1: Transfer operation is suspended during NACK reception (transfer suspension enabled)."]
    _1 = 1,
}
impl From<NACKE_A> for bool {
    #[inline(always)]
    fn from(variant: NACKE_A) -> Self {
        variant as u8 != 0
    }
}
impl NACKE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> NACKE_A {
        match self.bits {
            false => NACKE_A::_0,
            true => NACKE_A::_1,
        }
    }
    #[doc = "Transfer operation is not suspended during NACK reception (transfer suspension disabled)."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NACKE_A::_0
    }
    #[doc = "Transfer operation is suspended during NACK reception (transfer suspension enabled)."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NACKE_A::_1
    }
}
#[doc = "Field `NACKE` writer - NACK Reception Transfer Suspension Enable"]
pub type NACKE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, NACKE_A>;
impl<'a, REG, const O: u8> NACKE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transfer operation is not suspended during NACK reception (transfer suspension disabled)."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(NACKE_A::_0)
    }
    #[doc = "Transfer operation is suspended during NACK reception (transfer suspension enabled)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(NACKE_A::_1)
    }
}
#[doc = "Field `NFE` reader - Digital Noise Filter Circuit Enable"]
pub type NFE_R = crate::BitReader<NFE_A>;
#[doc = "Digital Noise Filter Circuit Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NFE_A {
    #[doc = "0: No digital noise filter circuit is used."]
    _0 = 0,
    #[doc = "1: A digital noise filter circuit is used."]
    _1 = 1,
}
impl From<NFE_A> for bool {
    #[inline(always)]
    fn from(variant: NFE_A) -> Self {
        variant as u8 != 0
    }
}
impl NFE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> NFE_A {
        match self.bits {
            false => NFE_A::_0,
            true => NFE_A::_1,
        }
    }
    #[doc = "No digital noise filter circuit is used."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NFE_A::_0
    }
    #[doc = "A digital noise filter circuit is used."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NFE_A::_1
    }
}
#[doc = "Field `NFE` writer - Digital Noise Filter Circuit Enable"]
pub type NFE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, NFE_A>;
impl<'a, REG, const O: u8> NFE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No digital noise filter circuit is used."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(NFE_A::_0)
    }
    #[doc = "A digital noise filter circuit is used."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(NFE_A::_1)
    }
}
#[doc = "Field `SCLE` reader - SCL Synchronous Circuit Enable"]
pub type SCLE_R = crate::BitReader<SCLE_A>;
#[doc = "SCL Synchronous Circuit Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCLE_A {
    #[doc = "0: No SCL synchronous circuit is used."]
    _0 = 0,
    #[doc = "1: An SCL synchronous circuit is used."]
    _1 = 1,
}
impl From<SCLE_A> for bool {
    #[inline(always)]
    fn from(variant: SCLE_A) -> Self {
        variant as u8 != 0
    }
}
impl SCLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SCLE_A {
        match self.bits {
            false => SCLE_A::_0,
            true => SCLE_A::_1,
        }
    }
    #[doc = "No SCL synchronous circuit is used."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SCLE_A::_0
    }
    #[doc = "An SCL synchronous circuit is used."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SCLE_A::_1
    }
}
#[doc = "Field `SCLE` writer - SCL Synchronous Circuit Enable"]
pub type SCLE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SCLE_A>;
impl<'a, REG, const O: u8> SCLE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No SCL synchronous circuit is used."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SCLE_A::_0)
    }
    #[doc = "An SCL synchronous circuit is used."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SCLE_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Timeout Function Enable"]
    #[inline(always)]
    pub fn tmoe(&self) -> TMOE_R {
        TMOE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Master Arbitration-Lost Detection Enable"]
    #[inline(always)]
    pub fn male(&self) -> MALE_R {
        MALE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - NACK Transmission Arbitration-Lost Detection Enable"]
    #[inline(always)]
    pub fn nale(&self) -> NALE_R {
        NALE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Slave Arbitration-Lost Detection Enable"]
    #[inline(always)]
    pub fn sale(&self) -> SALE_R {
        SALE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NACK Reception Transfer Suspension Enable"]
    #[inline(always)]
    pub fn nacke(&self) -> NACKE_R {
        NACKE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Digital Noise Filter Circuit Enable"]
    #[inline(always)]
    pub fn nfe(&self) -> NFE_R {
        NFE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SCL Synchronous Circuit Enable"]
    #[inline(always)]
    pub fn scle(&self) -> SCLE_R {
        SCLE_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timeout Function Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tmoe(&mut self) -> TMOE_W<ICFER_SPEC, 0> {
        TMOE_W::new(self)
    }
    #[doc = "Bit 1 - Master Arbitration-Lost Detection Enable"]
    #[inline(always)]
    #[must_use]
    pub fn male(&mut self) -> MALE_W<ICFER_SPEC, 1> {
        MALE_W::new(self)
    }
    #[doc = "Bit 2 - NACK Transmission Arbitration-Lost Detection Enable"]
    #[inline(always)]
    #[must_use]
    pub fn nale(&mut self) -> NALE_W<ICFER_SPEC, 2> {
        NALE_W::new(self)
    }
    #[doc = "Bit 3 - Slave Arbitration-Lost Detection Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sale(&mut self) -> SALE_W<ICFER_SPEC, 3> {
        SALE_W::new(self)
    }
    #[doc = "Bit 4 - NACK Reception Transfer Suspension Enable"]
    #[inline(always)]
    #[must_use]
    pub fn nacke(&mut self) -> NACKE_W<ICFER_SPEC, 4> {
        NACKE_W::new(self)
    }
    #[doc = "Bit 5 - Digital Noise Filter Circuit Enable"]
    #[inline(always)]
    #[must_use]
    pub fn nfe(&mut self) -> NFE_W<ICFER_SPEC, 5> {
        NFE_W::new(self)
    }
    #[doc = "Bit 6 - SCL Synchronous Circuit Enable"]
    #[inline(always)]
    #[must_use]
    pub fn scle(&mut self) -> SCLE_W<ICFER_SPEC, 6> {
        SCLE_W::new(self)
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
#[doc = "I2C Bus Function Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icfer::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icfer::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICFER_SPEC;
impl crate::RegisterSpec for ICFER_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`icfer::R`](R) reader structure"]
impl crate::Readable for ICFER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`icfer::W`](W) writer structure"]
impl crate::Writable for ICFER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ICFER to value 0x72"]
impl crate::Resettable for ICFER_SPEC {
    const RESET_VALUE: Self::Ux = 0x72;
}
