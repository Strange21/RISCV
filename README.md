# RISCV_RUST
## Developing Modern Low-Level Software Stacks to Fully Leverage the Capabilities of New-Age Hardware 

![image](https://media.github.boschdevcloud.com/user/30349/files/931a1472-c40e-4bf5-8987-2206701105b0)




- **ENABLERS** 
  - **SoC Partners**  
    -  [MINDGROVE](#mindgrove) 
    -  [SECURSI](#secursi)
    -  [NETRA SEMI](#netrasemi)
  - **Silicon Design Partners** 
    - [INCORE](#incore)
    - SHAKTI

## MINDGROVE
- Hardware Level
	-   Acquiring Hardware and Bootloader Customization
		-   Obtain the necessary hardware components.
		-   Test and modify the secondary bootloader to function as the primary bootloader.
	- System on Chip (SoC) Development for Custom Applications
		-  Heterogeneous Stack and SoC Design: Design and build a heterogeneous stack and SoC tailored for specific custom applications.
	  	- Accelerator Integration (anyone):
	  		-  GPU (Graphics Processing Unit): Handling complex graphical computations and parallel processing tasks.
	  		-  NPU (Neural Processing Unit): Accelerating neural network computations and AI-related tasks.
	  		-  VPU (Vision Processing Unit): Efficient processing of visual data, such as image and video analysis.
	  		-  DPU (Data Processing Unit): High-speed data processing and management, optimizing data-intensive operations.
	  		-  APU (Audio Processing Unit): Specialized audio processing tasks, enhancing audio quality and performance.
	- Additional Hardware Plans
		-   Understanding Existing Architecture: Analyze the architecture of existing heterogeneous hardware.
	  	-  Design Understanding: Study the design from Verilog or design level.
	  	-  Block Diagram Implementation: Implement the design at the block diagram level.
	  	-  Partnered Implementation: Collaborate with Mindgrove for custom building of heterogeneous SoC with specific peripherals.
-  Software Level
	-   SDK Completion
	      -  Development: Finalize the development of the Software Development Kit (SDK) in RUST, including libraries, tools, and documentation.
	      -  Testing: Verify the SDK's functionality, reliability, and compatibility through thorough testing.
	- Toolchain Development
		-  Building Tools: Develop a suite of tools around the SDK for development, debugging, and deployment
		-  Integration: Ensure seamless integration of these tools with existing development environments and workflows.
	- RISC-V Compiler Optimization
		- Compiler Enhancements: Optimize the RISC-V compiler to improve performance and efficiency of compiled code.
		- Performance Tuning: Conduct performance tuning and benchmarking to enhance execution speed and reduce resource consumption.
		- Feature Integration: Incorporate advanced compiler features and optimizations specific to the RISC-V architecture


## SECURSI
  - RustBoot as the primary bootloader without any workaround.
    - RustBoot will be utilized as the main bootloader, ensuring no alternative.
  - It will be tested on actual silicon, which is memory-mapped.
    - The bootloader will undergo testing on real silicon hardware where the memory is directly mapped, allowing for accurate and reliable performance evaluation.
  - The ROM address needs to be mapped for RustBoot.
    - Proper mapping of the ROM address is necessary for RustBoot to function correctly, ensuring that the bootloader can access and manage the memory locations effectively.
    - Multizone security.
  - Basic tools in and around it, in Rust.
    - Development and usage of fundamental tools related to RustBoot will be carried out in Rust, promoting a cohesive development environment.
    - Secure Debbuging, programmer, probe-rs support etc..
  - Closely working with the design team in software and hardware design.
    - Collaboration with the design team is essential, focusing on the integration of software and hardware design to ensure seamless functionality and performance.
  - Develop the software:
    - Compliance check up very small checkups -> CDAC:
      - Conduct minor compliance checks in collaboration with the Centre for Development of Advanced Computing (CDAC) to ensure adherence to standards.
    - Outside the country for standardization at an international level:
      - Aim for international standardization by seeking compliance and certification outside the country, enhancing global acceptance and reliability.
  - HSM needs to be certified. The first step is to get to the open-source version.
    - Certification of the Hardware Security Module (HSM) is crucial. The initial step involves developing and releasing an open-source version to facilitate widespread review and improvement.
  - The second step is to go through the design partnership to get the HSM taped out.
    - Following the open-source release, the next phase involves collaborating with design partners to finalize the HSM design and proceed with the tape-out process, which is the final step before manufacturing.
    - Understanding formal verification tools like prusti,Creusot Rust etc..
    - Nationally, there is an initiative to standardize the HSM in India. FIPS 140, FIPS 143 HSM standards:
    - There is a national effort in India to establish standardized HSMs following FIPS 140 and FIPS 143 standards, ensuring robust security measures and compliance.
    
## INCORE
  - Convert SDK from Shakti to Incore.
  - Integration with Secursi for HSM building.
  - Evaluation of core and getting it on FPGA.
  - Custom SoC from Incore which can serve purposes like RPi.
    - Utilize Azurite, which is positioned across M0-M4, Calcite, and Dolomite, as a RISC-V alternative to ARM's A5 & A55 for popular boards in the market like RPi, etc., as part of the first steps.

## NETRASEMI
  - Get both Netra A2000 with 4 TOPs and Netra R1000 with 2 TOPs.
  - Netra R1000 should be RISC-V designed with Shakti Core.
  - Get the C SDK as required from Netra Semi.
  - Translating there C SDK into a memory safe implementation using Rust.
  - This is more of a learning exercise up to this point.
  - Afterward, custom heterogeneous SoC and stack will be offered jointly as a solution, with a combination of software and hardware.

**NOTE :The above plans will be evolving, and the potential endgame will be enhanced.**

## MILESTONE 

![ROADMAP](https://media.github.boschdevcloud.com/user/30349/files/095a6178-a8cc-4ecd-85e3-75773f981709)

![Flow chart](https://media.github.boschdevcloud.com/user/30349/files/9f88066d-577c-4178-967e-965098c66495)
