#[doc = "Register `ICSR2` reader"]
pub type R = crate::R<ICSR2_SPEC>;
#[doc = "Register `ICSR2` writer"]
pub type W = crate::W<ICSR2_SPEC>;
#[doc = "Field `TMOF` reader - Timeout Detection Flag\n\nThe field is **modified** in some way after a read operation."]
pub type TMOF_R = crate::BitReader<TMOF_A>;
#[doc = "Timeout Detection Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TMOF_A {
    #[doc = "0: Timeout is not detected."]
    _0 = 0,
    #[doc = "1: Timeout is detected."]
    _1 = 1,
}
impl From<TMOF_A> for bool {
    #[inline(always)]
    fn from(variant: TMOF_A) -> Self {
        variant as u8 != 0
    }
}
impl TMOF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TMOF_A {
        match self.bits {
            false => TMOF_A::_0,
            true => TMOF_A::_1,
        }
    }
    #[doc = "Timeout is not detected."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TMOF_A::_0
    }
    #[doc = "Timeout is detected."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TMOF_A::_1
    }
}
#[doc = "Field `TMOF` writer - Timeout Detection Flag"]
pub type TMOF_W<'a, REG, const O: u8> = crate::BitWriter0C<'a, REG, O, TMOF_A>;
impl<'a, REG, const O: u8> TMOF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Timeout is not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TMOF_A::_0)
    }
    #[doc = "Timeout is detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TMOF_A::_1)
    }
}
#[doc = "Field `AL` reader - Arbitration-Lost Flag\n\nThe field is **modified** in some way after a read operation."]
pub type AL_R = crate::BitReader<AL_A>;
#[doc = "Arbitration-Lost Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AL_A {
    #[doc = "0: Arbitration is not lost."]
    _0 = 0,
    #[doc = "1: Arbitration is lost."]
    _1 = 1,
}
impl From<AL_A> for bool {
    #[inline(always)]
    fn from(variant: AL_A) -> Self {
        variant as u8 != 0
    }
}
impl AL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AL_A {
        match self.bits {
            false => AL_A::_0,
            true => AL_A::_1,
        }
    }
    #[doc = "Arbitration is not lost."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AL_A::_0
    }
    #[doc = "Arbitration is lost."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AL_A::_1
    }
}
#[doc = "Field `AL` writer - Arbitration-Lost Flag"]
pub type AL_W<'a, REG, const O: u8> = crate::BitWriter0C<'a, REG, O, AL_A>;
impl<'a, REG, const O: u8> AL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Arbitration is not lost."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(AL_A::_0)
    }
    #[doc = "Arbitration is lost."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(AL_A::_1)
    }
}
#[doc = "Field `START` reader - Start Condition Detection Flag\n\nThe field is **modified** in some way after a read operation."]
pub type START_R = crate::BitReader<START_A>;
#[doc = "Start Condition Detection Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum START_A {
    #[doc = "0: Start condition is not detected."]
    _0 = 0,
    #[doc = "1: Start condition is detected."]
    _1 = 1,
}
impl From<START_A> for bool {
    #[inline(always)]
    fn from(variant: START_A) -> Self {
        variant as u8 != 0
    }
}
impl START_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> START_A {
        match self.bits {
            false => START_A::_0,
            true => START_A::_1,
        }
    }
    #[doc = "Start condition is not detected."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == START_A::_0
    }
    #[doc = "Start condition is detected."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == START_A::_1
    }
}
#[doc = "Field `START` writer - Start Condition Detection Flag"]
pub type START_W<'a, REG, const O: u8> = crate::BitWriter0C<'a, REG, O, START_A>;
impl<'a, REG, const O: u8> START_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Start condition is not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(START_A::_0)
    }
    #[doc = "Start condition is detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(START_A::_1)
    }
}
#[doc = "Field `STOP` reader - Stop Condition Detection Flag\n\nThe field is **modified** in some way after a read operation."]
pub type STOP_R = crate::BitReader<STOP_A>;
#[doc = "Stop Condition Detection Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STOP_A {
    #[doc = "0: Stop condition is not detected."]
    _0 = 0,
    #[doc = "1: Stop condition is detected."]
    _1 = 1,
}
impl From<STOP_A> for bool {
    #[inline(always)]
    fn from(variant: STOP_A) -> Self {
        variant as u8 != 0
    }
}
impl STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> STOP_A {
        match self.bits {
            false => STOP_A::_0,
            true => STOP_A::_1,
        }
    }
    #[doc = "Stop condition is not detected."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == STOP_A::_0
    }
    #[doc = "Stop condition is detected."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == STOP_A::_1
    }
}
#[doc = "Field `STOP` writer - Stop Condition Detection Flag"]
pub type STOP_W<'a, REG, const O: u8> = crate::BitWriter0C<'a, REG, O, STOP_A>;
impl<'a, REG, const O: u8> STOP_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Stop condition is not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(STOP_A::_0)
    }
    #[doc = "Stop condition is detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(STOP_A::_1)
    }
}
#[doc = "Field `NACKF` reader - NACK Detection Flag\n\nThe field is **modified** in some way after a read operation."]
pub type NACKF_R = crate::BitReader<NACKF_A>;
#[doc = "NACK Detection Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NACKF_A {
    #[doc = "0: NACK is not detected."]
    _0 = 0,
    #[doc = "1: NACK is detected."]
    _1 = 1,
}
impl From<NACKF_A> for bool {
    #[inline(always)]
    fn from(variant: NACKF_A) -> Self {
        variant as u8 != 0
    }
}
impl NACKF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> NACKF_A {
        match self.bits {
            false => NACKF_A::_0,
            true => NACKF_A::_1,
        }
    }
    #[doc = "NACK is not detected."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NACKF_A::_0
    }
    #[doc = "NACK is detected."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NACKF_A::_1
    }
}
#[doc = "Field `NACKF` writer - NACK Detection Flag"]
pub type NACKF_W<'a, REG, const O: u8> = crate::BitWriter0C<'a, REG, O, NACKF_A>;
impl<'a, REG, const O: u8> NACKF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NACK is not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(NACKF_A::_0)
    }
    #[doc = "NACK is detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(NACKF_A::_1)
    }
}
#[doc = "Field `RDRF` reader - Receive Data Full Flag\n\nThe field is **modified** in some way after a read operation."]
pub type RDRF_R = crate::BitReader<RDRF_A>;
#[doc = "Receive Data Full Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDRF_A {
    #[doc = "0: ICDRR contains no receive data."]
    _0 = 0,
    #[doc = "1: ICDRR contains receive data."]
    _1 = 1,
}
impl From<RDRF_A> for bool {
    #[inline(always)]
    fn from(variant: RDRF_A) -> Self {
        variant as u8 != 0
    }
}
impl RDRF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RDRF_A {
        match self.bits {
            false => RDRF_A::_0,
            true => RDRF_A::_1,
        }
    }
    #[doc = "ICDRR contains no receive data."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RDRF_A::_0
    }
    #[doc = "ICDRR contains receive data."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RDRF_A::_1
    }
}
#[doc = "Field `RDRF` writer - Receive Data Full Flag"]
pub type RDRF_W<'a, REG, const O: u8> = crate::BitWriter0C<'a, REG, O, RDRF_A>;
impl<'a, REG, const O: u8> RDRF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ICDRR contains no receive data."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RDRF_A::_0)
    }
    #[doc = "ICDRR contains receive data."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RDRF_A::_1)
    }
}
#[doc = "Field `TEND` reader - Transmit End Flag\n\nThe field is **modified** in some way after a read operation."]
pub type TEND_R = crate::BitReader<TEND_A>;
#[doc = "Transmit End Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEND_A {
    #[doc = "0: Data is being transmitted."]
    _0 = 0,
    #[doc = "1: Data has been transmitted."]
    _1 = 1,
}
impl From<TEND_A> for bool {
    #[inline(always)]
    fn from(variant: TEND_A) -> Self {
        variant as u8 != 0
    }
}
impl TEND_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TEND_A {
        match self.bits {
            false => TEND_A::_0,
            true => TEND_A::_1,
        }
    }
    #[doc = "Data is being transmitted."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TEND_A::_0
    }
    #[doc = "Data has been transmitted."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TEND_A::_1
    }
}
#[doc = "Field `TEND` writer - Transmit End Flag"]
pub type TEND_W<'a, REG, const O: u8> = crate::BitWriter0C<'a, REG, O, TEND_A>;
impl<'a, REG, const O: u8> TEND_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Data is being transmitted."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TEND_A::_0)
    }
    #[doc = "Data has been transmitted."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TEND_A::_1)
    }
}
#[doc = "Field `TDRE` reader - Transmit Data Empty Flag"]
pub type TDRE_R = crate::BitReader<TDRE_A>;
#[doc = "Transmit Data Empty Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TDRE_A {
    #[doc = "0: ICDRT contains transmit data."]
    _0 = 0,
    #[doc = "1: ICDRT contains no transmit data."]
    _1 = 1,
}
impl From<TDRE_A> for bool {
    #[inline(always)]
    fn from(variant: TDRE_A) -> Self {
        variant as u8 != 0
    }
}
impl TDRE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TDRE_A {
        match self.bits {
            false => TDRE_A::_0,
            true => TDRE_A::_1,
        }
    }
    #[doc = "ICDRT contains transmit data."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TDRE_A::_0
    }
    #[doc = "ICDRT contains no transmit data."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TDRE_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - Timeout Detection Flag"]
    #[inline(always)]
    pub fn tmof(&self) -> TMOF_R {
        TMOF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Arbitration-Lost Flag"]
    #[inline(always)]
    pub fn al(&self) -> AL_R {
        AL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Start Condition Detection Flag"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Stop Condition Detection Flag"]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NACK Detection Flag"]
    #[inline(always)]
    pub fn nackf(&self) -> NACKF_R {
        NACKF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Receive Data Full Flag"]
    #[inline(always)]
    pub fn rdrf(&self) -> RDRF_R {
        RDRF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmit End Flag"]
    #[inline(always)]
    pub fn tend(&self) -> TEND_R {
        TEND_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmit Data Empty Flag"]
    #[inline(always)]
    pub fn tdre(&self) -> TDRE_R {
        TDRE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timeout Detection Flag"]
    #[inline(always)]
    #[must_use]
    pub fn tmof(&mut self) -> TMOF_W<ICSR2_SPEC, 0> {
        TMOF_W::new(self)
    }
    #[doc = "Bit 1 - Arbitration-Lost Flag"]
    #[inline(always)]
    #[must_use]
    pub fn al(&mut self) -> AL_W<ICSR2_SPEC, 1> {
        AL_W::new(self)
    }
    #[doc = "Bit 2 - Start Condition Detection Flag"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<ICSR2_SPEC, 2> {
        START_W::new(self)
    }
    #[doc = "Bit 3 - Stop Condition Detection Flag"]
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> STOP_W<ICSR2_SPEC, 3> {
        STOP_W::new(self)
    }
    #[doc = "Bit 4 - NACK Detection Flag"]
    #[inline(always)]
    #[must_use]
    pub fn nackf(&mut self) -> NACKF_W<ICSR2_SPEC, 4> {
        NACKF_W::new(self)
    }
    #[doc = "Bit 5 - Receive Data Full Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rdrf(&mut self) -> RDRF_W<ICSR2_SPEC, 5> {
        RDRF_W::new(self)
    }
    #[doc = "Bit 6 - Transmit End Flag"]
    #[inline(always)]
    #[must_use]
    pub fn tend(&mut self) -> TEND_W<ICSR2_SPEC, 6> {
        TEND_W::new(self)
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
#[doc = "I2C Bus Status Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icsr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icsr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICSR2_SPEC;
impl crate::RegisterSpec for ICSR2_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`icsr2::R`](R) reader structure"]
impl crate::Readable for ICSR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`icsr2::W`](W) writer structure"]
impl crate::Writable for ICSR2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x7f;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ICSR2 to value 0"]
impl crate::Resettable for ICSR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
