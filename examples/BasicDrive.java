/// The simplest subclass of SingDrive, meant to represent a drivetrain with two sets of in-line
/// wheels and motors (for examples, 3 motors on the left and 3 on the right).
public class BasicDrive extends SingDrive {
	
	
	 /// This is the essential constructor for BasicDrive. Its parameters are motor controller ports and the
	 /// driving speed constants.
	 ///
	 /// The number and position of motor controllers will likely change from year to year, and possibly in-season.
	 /// If so, the first several parameters and corresponding code will need to be edited accordingly.
	 /// 
	 /// @param leftMotor1 motor controller port # for one motor controller on the left drive train
	 /// @param leftMotor2 motor controller port # for one motor controller on the left drive train
	 /// @param rightMotor1 motor controller port # for one motor controller on the right drive train
	 /// @param rightMotor2 motor controller port # for one motor controller on the right drive train
	 /// 
	 /// @param slowSpeedConstant suggested values: 0.2 - 0.5
	 /// @param normalSpeedConstant suggested values: 0.6 - 1.0
	 /// @param fastSpeedConstant suggest value: 1.0
	 /// 
	 /// WARNING: This method will need to be changed if the number, type, or orientation of motor controllers changes!
	public BasicDrive(int leftMotor1, int leftMotor2, int leftMotor3, int rightMotor1, int rightMotor2, int rightMotor3,
	double slowSpeedConstant, double normalSpeedConstant, double fastSpeedConstant) {

		super(leftMotor1, leftMotor2, leftMotor3, rightMotor1, rightMotor2, rightMotor3, slowSpeedConstant, normalSpeedConstant, fastSpeedConstant);
	}


	/// This is the more basic constructor for BasicDrive. Its parameters are only motor controller ports, and they must
	/// correspond to the ports in the above constructor.
	/// 
	/// WARNING: This method will need to be changed if the number, type, or orientation of motor controllers changes!
	public BasicDrive(int leftMotor1, int leftMotor2, int leftMotor3, int rightMotor1, int rightMotor2,  int rightMotor3) {

		this(leftMotor1, leftMotor2, leftMotor3, rightMotor1, rightMotor2, rightMotor3,
		DEFAULT_SLOW_SPEED_CONSTANT, DEFAULT_NORMAL_SPEED_CONSTANT, DEFAULT_FAST_SPEED_CONSTANT);
	}



	/// Standard method for driving based on arcade, which means that one joystick axis controls translational speed and
	/// another controls rotational velocity.
	/// 
	/// @param vertical the forward/reverse constraint for robot movement
	/// @param rotation the turning constraint for robot movement
	/// @param horizontal the side-to-side constraint for robot movement: pass in 0.0, as BasicDrive cannot handle strafing movement
	/// @param poweredInputs pass true if inputs should be raised to the default power, thus improving sensitivity during slower driving
	/// @param speedMode controls the velocityMultiplier in order to scale motor velocity
	public void arcadeDrive(double vertical, double rotation, double horizontal, boolean poweredInputs, SpeedMode speedMode) {

		double forwardVelocity = vertical, rotationVelocity = -rotation;

		// Account for joystick drift.
		forwardVelocity = super.threshold(forwardVelocity);
		if (forwardVelocity != vertical) {
			rotationVelocity = threshold(rotationVelocity);
		}

		// If prompted, raise inputs to the default power.
		if (poweredInputs) {
			forwardVelocity = super.setInputToPower(forwardVelocity, DEFAULT_INPUT_POWER);
			rotationVelocity = super.setInputToPower(rotationVelocity, DEFAULT_INPUT_POWER);
		}

		// Change veloctiyMultiplier.
		setVelocityMultiplierBasedOnSpeedMode(speedMode);

		// If translation + rotation > 1, we will divide by this value, maximum, in order to only set motors to power -1 to 1.
		double maximum = Math.max(1, Math.abs(forwardVelocity) + Math.abs(rotationVelocity));

		double leftOutput = super.velocityMultiplier * (-forwardVelocity + rotationVelocity) / maximum;
		double rightOutput = super.velocityMultiplier * (forwardVelocity + rotationVelocity) / maximum;

		double velLeftOutput = (-forwardVelocity + rotationVelocity) / maximum;
		double velRightOutput = (-forwardVelocity + rotationVelocity) / maximum;

		SmartDashboard.putNumber("Output Left", leftOutput);
		SmartDashboard.putNumber("Output Right", rightOutput);

		// Drive the motors, and all subsequent motors through following.
		//super.m_leftMotor1.setSpeed(super.velocityMultiplier * (-forwardVelocity + rotationVelocity) / maximum);
		//super.m_rightMotor1.setSpeed(super.velocityMultiplier * (forwardVelocity + rotationVelocity) / maximum);

		super.m_leftMotor1.setVelocity(super.getVelocityOutput(leftOutput, speedMode));
		super.m_rightMotor1.setVelocity(super.getVelocityOutput(rightOutput, speedMode));
		
		// Drive the motors, and all subsequent motors through following.
		//super.m_leftMotor1.setSpeed(leftOutput);
		//super.m_rightMotor1.setSpeed(rightOutput);
	}
	
	///
	/// Standard method for driving based on tank, which means that one joystick controls the left
	/// drivetrain and another controls the right drivetrain.
	/// 
	/// @param left the constraint for the left drivetrain
	/// @param right the constraint for the right drivetrain
	/// @param horizontal the side-to-side constraint for robot movement: pass in 0.0, as BasicDrive cannot handle strafing movement
	/// @param poweredInputs pass true if inputs should be raised to the default power, thus improving sensitivity during slower driving
	/// @param speedMode controls the velocityMultiplier in order to scale motor velocity
	public void tankDrive(double left, double right, double horizontal, boolean poweredInputs, SpeedMode speedMode) {

		double leftVelocity = -left, rightVelocity = -right;

		// Account for joystick drift.
		leftVelocity = threshold(leftVelocity);
		rightVelocity = threshold(rightVelocity);

		// If prompted, raise inputs to the default power.
		if (poweredInputs) {
			leftVelocity = super.setInputToPower(leftVelocity, DEFAULT_INPUT_POWER);
			rightVelocity = super.setInputToPower(rightVelocity, DEFAULT_INPUT_POWER);
		}
		
		// Change velocityMultiplier.
		setVelocityMultiplierBasedOnSpeedMode(speedMode);
		
		// If a velocity > 1, we will divide by this value, maximum, in order to only set motors to power -1 to 1.
		double leftMaximum = Math.max(1, Math.abs(leftVelocity));
		double rightMaximum = Math.max(1, Math.abs(rightVelocity));

		double velRightOut = rightVelocity / rightMaximum;
		double velLeftOut = leftVelocity / leftMaximum;

		SmartDashboard.putNumber("Left Output", super.velocityMultiplier * leftVelocity / leftMaximum);
		SmartDashboard.putNumber("Right Output", -super.velocityMultiplier * rightVelocity / rightMaximum);
		// Drive the motors, and all subsequent motors through following.
		super.m_leftMotor1.setSpeed(super.velocityMultiplier * leftVelocity / leftMaximum);
		super.m_rightMotor1.setSpeed(-super.velocityMultiplier * rightVelocity / rightMaximum);

		//super.m_rightMotor1.setVelocity(super.getVelocityOutput(velRightOut, speedMode));
		//super.m_leftMotor1.setVelocity(super.getVelocityOutput(velLeftOut, speedMode));

	}
}
