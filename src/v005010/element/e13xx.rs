//! X12 data elements 1300-1399.

crate::code_enum!(
    /// **1300** Service, Promotion, Allowance, or Charge Code
    ///
    /// - Data element: 1300
    /// - Type: Identifier (ID)
    /// - Length: min 4, max 4
    ///
    /// Code identifying the service, promotion, allowance, or charge. Common codes
    /// named below; any other round-trips as `Unknown`.
    E1300 {
        /// Base Charge
        "A520" => A520,
        /// COD Amount
        "B230" => B230,
        /// Delivery
        "C040" => C040,
        /// Freight
        "D240" => D240,
        /// Fuel Charge
        "D260" => D260,
        /// Handling
        "D500" => D500,
        /// Insurance Fee
        "D920" => D920,
        /// Loading (Labor Charges)
        "E400" => E400,
        /// Promotional Allowance
        "F800" => F800,
        /// Service Charge
        "G740" => G740,
        /// Storage
        "H430" => H430,
        /// Surcharge
        "H550" => H550,
        /// Unloading (Labor Charges)
        "I380" => I380,
    }
);

crate::code_enum!(
    /// **1321** Condition Indicator
    ///
    /// - Data element: 1321
    /// - Type: Identifier (ID)
    /// - Length: min 2, max 3
    ///
    /// Condition Indicator. Code values verified against the Stedi X12 reference.
    E1321 {
        /// Requested
        "00" => N00,
        /// In Progress
        "000" => N000,
        /// Automated Export System - Post Departure Authorized Special Status (AES-PASS) Standard
        "0A" => N0A,
        /// Automated Export System - Post Departure Authorized Special Status (AES-PASS) Expanded
        "0B" => N0B,
        /// Automated Export System - Post Departure Authorized Special Status (AES-PASS) Post Departure
        "0C" => N0C,
        /// Facility's Emergency Response Plan Includes Information on Emergency Health Care
        "0D" => N0D,
        /// Facility's Emergency Response Plan Includes Procedures for Informing Public and Local Agencies Responsible for Responding to an Accidental Release
        "0E" => N0E,
        /// Facility has a Clean Air Act Title V Operating Permit
        "0F" => N0F,
        /// Facility has a Written Emergency Response Plan
        "0G" => N0G,
        /// Facility has Reportable Accidents
        "0H" => N0H,
        /// Facility is Covered by the Emergency Planning and Community Right to Know Act Section 302
        "0I" => N0I,
        /// Facility is Covered by the Occupational Safety and Health Act (OSHA) Process Safety Management Standard
        "0J" => N0J,
        /// Facility is Included in the Community Emergency Response Plan
        "0K" => N0K,
        /// Hazardous Waste Mixed with Resource Conservation Recovery Act (RCRA)-Radioactive Material
        "0L" => N0L,
        /// Offsite Responders Notified
        "0M" => N0M,
        /// Precipitation Present
        "0N" => N0N,
        /// Disabled Veteran
        "0O" => N0O,
        /// Servicer has Advanced Funds to Pay for Delinquent Taxes on Non-escrowed Mortgage
        "0P" => N0P,
        /// Property Has Fire Insurance Only that was not Lender Placed
        "0Q" => N0Q,
        /// Reported but Unconfirmed
        "0R" => N0R,
        /// Has Smoke Alarms
        "0S" => N0S,
        /// Operates as a Holding Company
        "0T" => N0T,
        /// Optimum
        "0U" => N0U,
        /// Renewed
        "0V" => N0V,
        /// Highest Educational Level
        "0W" => N0W,
        /// Principal Certificate
        "0X" => N0X,
        /// Inservice Education Completed
        "0Y" => N0Y,
        /// Main Assignment
        "0Z" => N0Z,
        /// Patient was admitted to a hospital
        "01" => N01,
        /// Patient is receiving anti-fungal therapy
        "1A" => N1A,
        /// Property is occupied by owner
        "1B" => N1B,
        /// Property is occupied by tenant
        "1C" => N1C,
        /// Property is vacant
        "1D" => N1D,
        /// Location is urban
        "1E" => N1E,
        /// Location is suburban
        "1F" => N1F,
        /// Location is rural
        "1G" => N1G,
        /// Built-up over 75%
        "1H" => N1H,
        /// Built-up 25 - 75%
        "1I" => N1I,
        /// Built-up under 25%
        "1J" => N1J,
        /// Growth rate is rapid
        "1K" => N1K,
        /// Class I-Left
        "1L" => N1L,
        /// Growth rate is stable
        "1M" => N1M,
        /// Growth rate is slow
        "1N" => N1N,
        /// Property values are increasing
        "1O" => N1O,
        /// Property values are stable
        "1P" => N1P,
        /// Property values are declining
        "1Q" => N1Q,
        /// Class I-Right
        "1R" => N1R,
        /// Demand or supply is in shortage
        "1S" => N1S,
        /// Demand or supply is in balance
        "1T" => N1T,
        /// Demand or supply is over supply
        "1U" => N1U,
        /// Marketing time is under 3 months
        "1V" => N1V,
        /// Marketing time is 3 to 6 months
        "1W" => N1W,
        /// Marketing time is over 6 months
        "1X" => N1X,
        /// Predominant occupancy is the owner
        "1Y" => N1Y,
        /// Predominant occupancy is the tenant
        "1Z" => N1Z,
        /// Patient was bed confined before the ambulance service
        "02" => N02,
        /// Patient is receiving oral anti-fungal therapy
        "2A" => N2A,
        /// Predominant occupancy is vacant (0-5%)
        "2B" => N2B,
        /// Predominant occupancy is vacant (over 5%)
        "2C" => N2C,
        /// Developer or builder is in control of the Home Owners Association
        "2D" => N2D,
        /// Site is a corner lot
        "2E" => N2E,
        /// Zoning compliance is legal
        "2F" => N2F,
        /// Zoning compliance is legal nonconforming (grandfather use)
        "2G" => N2G,
        /// Zoning compliance is illegal
        "2H" => N2H,
        /// There is no zoning
        "2I" => N2I,
        /// Highest and best use as improved is the present use
        "2J" => N2J,
        /// Highest and best use as improved is other use
        "2K" => N2K,
        /// Class II-Left
        "2L" => N2L,
        /// Property is located in a Federal Emergency Management Administration special flood hazard area
        "2M" => N2M,
        /// Appraisal is made "as is"
        "2N" => N2N,
        /// Appraisal is made subject to the repairs, alterations, inspections, or conditions listed
        "2O" => N2O,
        /// Appraisal is made subject to the completion per plans and specifications
        "2P" => N2P,
        /// Project type is planned unit development (PUD)
        "2Q" => N2Q,
        /// Class II-Right
        "2R" => N2R,
        /// Project type is condominium
        "2S" => N2S,
        /// Property rights are fee simple
        "2T" => N2T,
        /// Property rights are leasehold
        "2U" => N2U,
        /// Supervisor appraiser inspected the property per supervisory appraiser's certification
        "2V" => N2V,
        /// Property was sold within last 12 months
        "2W" => N2W,
        /// Appraiser signed statement of limiting conditions and disclaimer
        "2X" => N2X,
        /// Ownership interest in a property
        "2Y" => N2Y,
        /// Termination
        "2Z" => N2Z,
        /// Patient was bed confined after the ambulance service
        "03" => N03,
        /// Patient is receiving topical anti-fungal therapy
        "3A" => N3A,
        /// Points Paid by Seller
        "3B" => N3B,
        /// Points Paid by Buyer
        "3C" => N3C,
        /// Seller Concession
        "3D" => N3D,
        /// Letter of Certification
        "3E" => N3E,
        /// Verbal Report Needed
        "3F" => N3F,
        /// Any Relationship Between Owner and Occupant
        "3G" => N3G,
        /// Map and Directions to Remote Properties to Follow
        "3H" => N3H,
        /// Ground Lease to Follow
        "3I" => N3I,
        /// Disclosure Statement to Follow
        "3J" => N3J,
        /// Copy of Property Listing to Follow
        "3K" => N3K,
        /// Class III-Left
        "3L" => N3L,
        /// Copy of Title Report Plat Map to Follow
        "3M" => N3M,
        /// Property Tax Bill to Follow
        "3N" => N3N,
        /// Engineering or Soil Report to Follow
        "3O" => N3O,
        /// Sales Contract Available
        "3P" => N3P,
        /// Leave Will be Taken
        "3Q" => N3Q,
        /// Class III-Right
        "3R" => N3R,
        /// Approved
        "3S" => N3S,
        /// Balance Sheet does not balance
        "3T" => N3T,
        /// Banking done through Parent Company
        "3U" => N3U,
        /// Banking done through Related Concern
        "3V" => N3V,
        /// Banking done through Subsidiary
        "3W" => N3W,
        /// Can not determine if subject engaged in business
        "3X" => N3X,
        /// Deteriorated
        "3Y" => N3Y,
        /// Detrimental Information Received
        "3Z" => N3Z,
        /// Patient was moved by stretcher
        "04" => N04,
        /// Services are rendered within Hospice-elected period of coverage
        "4A" => N4A,
        /// Accidents
        "4B" => N4B,
        /// Account Representative Transfer
        "4C" => N4C,
        /// Additional Coverage
        "4D" => N4D,
        /// Advice to Stop
        "4E" => N4E,
        /// Agent Replacement
        "4F" => N4F,
        /// Backup Withholding
        "4G" => N4G,
        /// Current Employer
        "4H" => N4H,
        /// Current Occupation
        "4I" => N4I,
        /// Employer Reimbursement
        "4J" => N4J,
        /// Employee Retirement Income Security Act (ERISA)
        "4K" => N4K,
        /// Expected Changes
        "4L" => N4L,
        /// Experimental
        "4M" => N4M,
        /// Foreign Flight
        "4N" => N4N,
        /// Future Involvement
        "4O" => N4O,
        /// Grounding, Fine, Reprimand
        "4P" => N4P,
        /// Group Disability Insurance Conversion
        "4Q" => N4Q,
        /// Group Disability Insurance Offset
        "4R" => N4R,
        /// Group Disability Insurance Participation
        "4S" => N4S,
        /// Group Disability Insurance Top Up
        "4T" => N4T,
        /// Home Employment
        "4U" => N4U,
        /// Information Omitted
        "4V" => N4V,
        /// Injury Benefits
        "4W" => N4W,
        /// Issue at Higher Premiums
        "4X" => N4X,
        /// Issue With Exclusions
        "4Y" => N4Y,
        /// Issue Without Benefits
        "4Z" => N4Z,
        /// Patient was unconscious or in shock
        "05" => N05,
        /// Treatment is rendered related to the terminal illness
        "5A" => N5A,
        /// Certified Aftermarket Parts Association (CAPA) Only
        "5B" => N5B,
        /// Certified Aftermarket Parts Association (CAPA) Preferred
        "5C" => N5C,
        /// Juvenile Seen
        "5D" => N5D,
        /// Medical Treatment
        "5E" => N5E,
        /// Military Aviation
        "5F" => N5F,
        /// New Group
        "5G" => N5G,
        /// Other Coverage Offset
        "5H" => N5H,
        /// Other Principals Being Insured
        "5I" => N5I,
        /// Owner Active in Business
        "5J" => N5J,
        /// Payroll Deduction
        "5K" => N5K,
        /// Prepaid
        "5L" => N5L,
        /// Previous Application
        "5M" => N5M,
        /// Primary Occupation
        "5N" => N5N,
        /// Racing Accident
        "5O" => N5O,
        /// Replacement
        "5P" => N5P,
        /// Resides With Applicant
        "5Q" => N5Q,
        /// Gender Distinct
        "5R" => N5R,
        /// Sibling Coverage
        "5S" => N5S,
        /// Sickness Benefits
        "5T" => N5T,
        /// Special Dating
        "5U" => N5U,
        /// Spousal Consent
        "5V" => N5V,
        /// Suitability Analysis
        "5W" => N5W,
        /// Suitable for Coverage
        "5X" => N5X,
        /// Taxable
        "5Y" => N5Y,
        /// This Company Replacement
        "5Z" => N5Z,
        /// Patient was transported in an emergency situation
        "06" => N06,
        /// Treatment is rendered by a Hospice employed physician
        "6A" => N6A,
        /// United States Citizen
        "6B" => N6B,
        /// Permanent Resident Alien
        "6C" => N6C,
        /// Borrower is First Time Homebuyer
        "6D" => N6D,
        /// Unemployment Claims
        "6E" => N6E,
        /// Unemployment Insurance Eligibility
        "6F" => N6F,
        /// Work Status
        "6G" => N6G,
        /// Workers Compensation Eligible
        "6H" => N6H,
        /// Factored on Recourse Basis
        "6I" => N6I,
        /// Factored with Advances
        "6J" => N6J,
        /// Figures are Actual
        "6K" => N6K,
        /// Figures are Anticipated
        "6L" => N6L,
        /// Figures are Estimated
        "6M" => N6M,
        /// Figures are Modified
        "6N" => N6N,
        /// Figures are Projected
        "6O" => N6O,
        /// Government Business Number Unavailable
        "6P" => N6P,
        /// Goodwill Origin Purchased from Bankrupt Company
        "6Q" => N6Q,
        /// Goodwill Origin Rented
        "6R" => N6R,
        /// Has no ownership
        "6S" => N6S,
        /// Improved
        "6T" => N6T,
        /// Intangibles breakdown available
        "6U" => N6U,
        /// Intangibles include Organizational Expense
        "6V" => N6V,
        /// Intercompany relations consist of Loans and Advances
        "6W" => N6W,
        /// Intercompany relations consist of Merchandise Transactions
        "6X" => N6X,
        /// Intercompany relations consist of Service Transactions
        "6Y" => N6Y,
        /// Local banking utilized on a transfer account basis
        "6Z" => N6Z,
        /// Patient had to be physically restrained
        "07" => N07,
        /// Treatment is rendered by a private attending physician
        "7A" => N7A,
        /// Medications Ordered are being Administered Intramuscularly
        "7B" => N7B,
        /// Medications Ordered are being Administered Intravenously
        "7C" => N7C,
        /// Medications Ordered are being Administered Orally
        "7D" => N7D,
        /// Maintains no Inventory
        "7E" => N7E,
        /// Medications Ordered are being Administered Subcutaneously
        "7F" => N7F,
        /// Majority
        "7G" => N7G,
        /// Marketable Securities valued at cost
        "7H" => N7H,
        /// Marketable Securities valued at lower of cost or market
        "7I" => N7I,
        /// Interior Access Denied
        "7J" => N7J,
        /// Repairs are Recommended
        "7K" => N7K,
        /// Loan Originated under Shared Equity Plan
        "7L" => N7L,
        /// Title and or Legal Issues Exist
        "7M" => N7M,
        /// Environmental Issues Exist
        "7N" => N7N,
        /// Property is Listed As Is
        "7O" => N7O,
        /// Property is Listed as Repaired
        "7P" => N7P,
        /// Vacancy Rate is Greater Than 5 Percent to 10 Percent
        "7Q" => N7Q,
        /// Vacancy Rate is Greater Than 10 Percent to 20 Percent
        "7R" => N7R,
        /// Vacancy Rate is Greater Than 20 Percent
        "7S" => N7S,
        /// Most Comparable Property
        "7T" => N7T,
        /// Anticipate Issues which Affect Ability to Secure Financing
        "7U" => N7U,
        /// Points are Paid by Seller
        "7V" => N7V,
        /// Property Covered by Flood Insurance Policy
        "7W" => N7W,
        /// Property Covered by Earthquake Insurance Policy
        "7X" => N7X,
        /// Points are Negotiable
        "7Y" => N7Y,
        /// Property is Currently Listed with a Real Estate Firm
        "7Z" => N7Z,
        /// Patient had visible hemorrhaging
        "08" => N08,
        /// Treatment is curative
        "8A" => N8A,
        /// Income or Assets of Another Used
        "8B" => N8B,
        /// Disclosure of Someone Else's Liabilities Required
        "8C" => N8C,
        /// Property Improvements "to be made"
        "8D" => N8D,
        /// Property Improvements "have been made"
        "8E" => N8E,
        /// Distant Suburban
        "8F" => N8F,
        /// Self Employed
        "8G" => N8G,
        /// Liability to be Satisfied
        "8H" => N8H,
        /// Are Assets/Liabilities Reported Jointly
        "8I" => N8I,
        /// Location is Farm
        "8J" => N8J,
        /// Location is Resort
        "8K" => N8K,
        /// Shortage Exist for Competing Listings
        "8L" => N8L,
        /// Competing Listings are in Balance
        "8M" => N8M,
        /// Oversupply Exist for Competing Listings
        "8N" => N8N,
        /// Incentives are Offered
        "8O" => N8O,
        /// Listed Property has been Inspected
        "8P" => N8P,
        /// Sale Property has been Inspected
        "8Q" => N8Q,
        /// General Marketing Condition is Depressed
        "8R" => N8R,
        /// General Marketing Condition is Slow
        "8S" => N8S,
        /// General Marketing Condition is Static
        "8T" => N8T,
        /// General Marketing Condition is Improving
        "8U" => N8U,
        /// General Marketing Condition is Excellent
        "8V" => N8V,
        /// Employment Conditions are Stable
        "8W" => N8W,
        /// Employment Conditions are Declining
        "8X" => N8X,
        /// Employment Conditions are Increasing
        "8Y" => N8Y,
        /// Overimprovement Condition Exists
        "8Z" => N8Z,
        /// Ambulance service was medically necessary
        "09" => N09,
        /// Treatment is Palliative
        "9A" => N9A,
        /// Involuntary Committal
        "9B" => N9B,
        /// Lack of Available Equipment
        "9C" => N9C,
        /// Lack of Appropriate Facility within Reasonable Distance to Treat Patient in the Event of Complications
        "9D" => N9D,
        /// Sudden Onset of Disorientation
        "9E" => N9E,
        /// Sudden Onset of Severe, Incapacitating Pain
        "9F" => N9F,
        /// Continuous Hemorrhage from any Site with Abnormal Lab Values
        "9G" => N9G,
        /// Patient Requires Intensive IV Therapy
        "9H" => N9H,
        /// Patient Requires Volume Expanders
        "9I" => N9I,
        /// Patient Requires Protective Isolation
        "9J" => N9J,
        /// Patient Requires Frequent Monitoring
        "9K" => N9K,
        /// Patient Requires Extended Post-operative Observation
        "9L" => N9L,
        /// Foreclosure Proceedings Have Begun
        "9M" => N9M,
        /// Underimprovement Condition Exists
        "9N" => N9N,
        /// Marketability of Property is Excellent
        "9O" => N9O,
        /// Marketability of Property is Good
        "9P" => N9P,
        /// Marketability of Property is Fair
        "9Q" => N9Q,
        /// Marketability of Property is Poor
        "9R" => N9R,
        /// Fees are Current
        "9S" => N9S,
        /// Fees Include Tennis
        "9T" => N9T,
        /// Fees Include Pool
        "9U" => N9U,
        /// Fees Include Insurance
        "9V" => N9V,
        /// Fees Include Landscape
        "9W" => N9W,
        /// Fees Include Other Amenities
        "9X" => N9X,
        /// Most Likely Buyer is Owner Occupant
        "9Y" => N9Y,
        /// Most Likely Buyer is Investor
        "9Z" => N9Z,
        /// Patient is ambulatory
        "10" => N10,
        /// Ambulation is Impaired and Walking Aid is Used for Therapy or Mobility
        "11" => N11,
        /// Patient is confined to a bed or chair
        "12" => N12,
        /// Patient is Confined to a Room or an Area Without Bathroom Facilities
        "13" => N13,
        /// Ambulation is Impaired and Walking Aid is Used for Mobility
        "14" => N14,
        /// Patient Condition Requires Positioning of the Body or Attachments Which Would Not be Feasible With the Use of an Ordinary Bed
        "15" => N15,
        /// Patient needs a trapeze bar to sit up due to respiratory condition or change body positions for other medical reasons
        "16" => N16,
        /// Patient's Ability to Breathe is Severely Impaired
        "17" => N17,
        /// Patient condition requires frequent and/or immediate changes in body positions
        "18" => N18,
        /// Patient can operate controls
        "19" => N19,
        /// Siderails Are to be Attached to a Hospital Bed Owned by the Beneficiary
        "20" => N20,
        /// Patient owns equipment
        "21" => N21,
        /// Mattress or Siderails are Being Used with Prescribed Medically Necessary Hospital Bed Owned by the Beneficiary
        "22" => N22,
        /// Patient Needs Lift to Get In or Out of Bed or to Assist in Transfer from Bed to Wheelchair
        "23" => N23,
        /// Patient has an orthopedic impairment requiring traction equipment which prevents ambulation during period of use
        "24" => N24,
        /// Item has been prescribed as part of a planned regimen of treatment in patient home
        "25" => N25,
        /// Patient is highly susceptible to decubitus ulcers
        "26" => N26,
        /// Patient or a care-giver has been instructed in use of equipment
        "27" => N27,
        /// Patient has poor diabetic control
        "28" => N28,
        /// A 6-7 hour nocturnal study documents 30 episodes of apnea each lasting more than 10 seconds
        "29" => N29,
        /// Without the equipment, the patient would require surgery
        "30" => N30,
        /// Patient has had a total knee replacement
        "31" => N31,
        /// Patient has intractable lymphedema of the extremities
        "32" => N32,
        /// Patient is in a nursing home
        "33" => N33,
        /// Patient is conscious
        "34" => N34,
        /// This Feeding is the Only Form of Nutritional Intake for This Patient
        "35" => N35,
        /// Patient was administered premix
        "36" => N36,
        /// Oxygen delivery equipment is stationary
        "37" => N37,
        /// Certification signed by the physician is on file at the supplier's office
        "38" => N38,
        /// Patient Has Mobilizing Respiratory Tract Secretions
        "39" => N39,
        /// Patient or Caregiver is Capable of Using the Equipment Without Technical or Professional Supervision
        "40" => N40,
        /// Patient or Caregiver is Unable to Propel or Lift a Standard Weight Wheelchair
        "41" => N41,
        /// Patient Requires Leg Elevation for Edema or Body Alignment
        "42" => N42,
        /// Patient Weight or Usage Needs Necessitate a Heavy Duty Wheelchair
        "43" => N43,
        /// Patient Requires Reclining Function of a Wheelchair
        "44" => N44,
        /// Patient is Unable to Operate a Wheelchair Manually
        "45" => N45,
        /// Patient or Caregiver Requires Side Transfer into Wheelchair, Commode or Other
        "46" => N46,
        /// Advertisement Run Condition
        "47" => N47,
        /// Individual Paid for Last Day Worked
        "48" => N48,
        /// Full Wages Paid for Date of Injury
        "49" => N49,
        /// Citation or Ticket Issued
        "50" => N50,
        /// Individual is Member of Policyholder's Household
        "51" => N51,
        /// Individual Permitted to Use Vehicle
        "52" => N52,
        /// Individual Wore Seatbelt
        "53" => N53,
        /// Child Restraint Device in Vehicle
        "54" => N54,
        /// Child Restraint Device Used
        "55" => N55,
        /// Individual Injured
        "56" => N56,
        /// Individual Transported to Another Location
        "57" => N57,
        /// Durable Medical Equipment (DME) Purchased New
        "58" => N58,
        /// Durable Medical Equipment (DME) Is Under Warranty
        "59" => N59,
        /// Transportation Was To the Nearest Facility
        "60" => N60,
        /// Employee is Exempt
        "61" => N61,
        /// Claimant is Covered on the Employer's Long-term Disability Plan
        "62" => N62,
        /// Employee's Job Responsibilities Changed Due to the Disabling Condition
        "63" => N63,
        /// Employer Has a Return to Work Policy for Disabled Employees
        "64" => N64,
        /// Open
        "65" => N65,
        /// Normal
        "66" => N66,
        /// Closed-moderate
        "67" => N67,
        /// Severe
        "68" => N68,
        /// Moderate
        "69" => N69,
        /// Straight
        "70" => N70,
        /// Convex
        "71" => N71,
        /// Concave
        "72" => N72,
        /// Double Protrusion
        "73" => N73,
        /// No Crossbite
        "74" => N74,
        /// Posterior
        "75" => N75,
        /// Anterior
        "76" => N76,
        /// Maxillary
        "77" => N77,
        /// Mandibular
        "78" => N78,
        /// Right
        "79" => N79,
        /// Left
        "80" => N80,
        /// Maxillary Moderate
        "81" => N81,
        /// Mandibular Moderate
        "82" => N82,
        /// Maxillary Severe
        "83" => N83,
        /// Mandibular Severe
        "84" => N84,
        /// Income Has Been Verified
        "85" => N85,
        /// Person Has Been Interviewed
        "86" => N86,
        /// Rent Has Been Verified
        "87" => N87,
        /// Employer Has Been Verified
        "88" => N88,
        /// Position Has Been Verified
        "89" => N89,
        /// Inquiry Has Been Verified
        "90" => N90,
        /// Outstanding Judgments
        "91" => N91,
        /// Declared Bankruptcy in Past 7 Years
        "92" => N92,
        /// Foreclosure or Deed in Lieu in Past 7 Years
        "93" => N93,
        /// Party to Lawsuit
        "94" => N94,
        /// Obligated on a Loan Foreclosed, Deed in Lieu of Judgment
        "95" => N95,
        /// Currently Delinquent or in Default
        "96" => N96,
        /// Obligated to Pay Alimony, Child Support or Maintenance
        "97" => N97,
        /// Part of Down Payment Borrowed
        "98" => N98,
        /// Co-maker or Endorser on a Note
        "99" => N99,
        /// Liability Coverage Will Transfer
        "A0" => A0,
        /// Most Likely Buyer is Other Person or Entity
        "A1" => A1,
        /// Potential Financing is Fannie Mae
        "A2" => A2,
        /// Suppress Paper Endorsement
        "A3" => A3,
        /// Do Not Suppress Paper Endorsement
        "A4" => A4,
        /// Escrow
        "A5" => A5,
        /// Teaching Minor
        "A6" => A6,
        /// Sub-servicer Submitted
        "A7" => A7,
        /// First Mortgage
        "A8" => A8,
        /// Second Mortgage
        "A9" => A9,
        /// Amputation
        "AA" => Aa,
        /// Address Skip Begin
        "AB" => Ab,
        /// Address Corrected
        "AC" => Ac,
        /// Automatic Drill Time Calculated
        "AD" => Ad,
        /// Automatic Edging Time Calculated
        "AE" => Ae,
        /// Automatically Select
        "AF" => Af,
        /// Accepting Family Members
        "AFM" => Afm,
        /// Agitated
        "AG" => Ag,
        /// Automatically Search and List
        "AH" => Ah,
        /// Address Incorrect
        "AI" => Ai,
        /// Assumable
        "AJ" => Aj,
        /// Potential Financing is Cash
        "AK" => Ak,
        /// Ambulation Limitations
        "AL" => Al,
        /// Potential Financing is Outside Lender
        "AM" => Am,
        /// Address Incomplete
        "AN" => An,
        /// Accept Certification without Changes
        "AO" => Ao,
        /// Alley is Public
        "AP" => Ap,
        /// Potential Financing is Federal Housing Administration
        "AQ" => Aq,
        /// Address Skip Resolved
        "AR" => Ar,
        /// Address Skip Exhaust
        "AS" => As,
        /// Accept Statement of Limiting Conditions without Changes
        "AT" => At,
        /// Automatic Underside Time Calculated
        "AU" => Au,
        /// Available - Not Used
        "AV" => Av,
        /// Accept Certification with Changes
        "AW" => Aw,
        /// Accept Statement of Limiting Conditions with Changes
        "AX" => Ax,
        /// Adjacent Track Occupied
        "AY" => Ay,
        /// Potential Financing is Veterans Affairs
        "AZ" => Az,
        /// Uninsured Motorist Coverage Will Transfer
        "B0" => B0,
        /// Mortgage in Foreclosure
        "B1" => B1,
        /// Real Estate Owned (REO) Mortgage
        "B2" => B2,
        /// Potential Financing is Contract for Deed
        "B3" => B3,
        /// Only the Exterior has been Inspected
        "B4" => B4,
        /// Real Estate Owned Property or Foreclosure Property
        "B5" => B5,
        /// Number of Comparable Listings is Normal
        "B6" => B6,
        /// Number of Comparable Listings is an Oversupply
        "B7" => B7,
        /// Number of Comparable Listings is a Shortage
        "B8" => B8,
        /// Property Management Expenses Outstanding
        "B9" => B9,
        /// Borrower Letter Attempt
        "BA" => Ba,
        /// Building or Mobile Home is in a Coastal Barrier Resources Area
        "BB" => Bb,
        /// Borrower Telephone Contact
        "BC" => Bc,
        /// Business Pending
        "BD" => Bd,
        /// Borrower Letter Contact
        "BE" => Be,
        /// Marketable Securities valued at market
        "BF" => Bf,
        /// Appropriate Improvement Condition Exists
        "BG" => Bg,
        /// Name unknown to local authorities
        "BH" => Bh,
        /// No manufacturing done on Premises
        "BI" => Bi,
        /// Occasional
        "BJ" => Bj,
        /// Officer or owner in other Businesses
        "BK" => Bk,
        /// Bowel Limitations, Bladder Limitations, or both (Incontinence)
        "BL" => Bl,
        /// Old
        "BM" => Bm,
        /// Operates on part time basis
        "BN" => Bn,
        /// Parent Financial Statement Used
        "BO" => Bo,
        /// Borrower Payment Received
        "BP" => Bp,
        /// Beneficiary is Partially Dependent
        "BPD" => Bpd,
        /// Product Information Available
        "BQ" => Bq,
        /// Bedrest BRP (Bathroom Privileges)
        "BR" => Br,
        /// Revenue derived from Commissions
        "BS" => Bs,
        /// Borrower Telephone Attempt
        "BT" => Bt,
        /// Beneficiary is Totally Dependent
        "BTD" => Btd,
        /// Revenue derived from Donations
        "BU" => Bu,
        /// Revenue derived from Fees
        "BV" => Bv,
        /// Revenue derived from Grants
        "BW" => Bw,
        /// Revenue derived from Taxes
        "BX" => Bx,
        /// Sprinkler Equipped
        "BY" => By,
        /// Statement requested from Government Registry
        "BZ" => Bz,
        /// Collision Coverage Will Transfer
        "C0" => C0,
        /// Advances From Property Management Expenses Outstanding
        "C1" => C1,
        /// Final Demand Letter Sent
        "C2" => C2,
        /// Lender Request for Assistance
        "C3" => C3,
        /// Mortgage has Lender-purchased Mortgage Insurance
        "C4" => C4,
        /// Insufficient Funds
        "C5" => C5,
        /// Credit Enhanced Mortgage
        "C6" => C6,
        /// Corporate Appointment
        "C7" => C7,
        /// Special Servicing Required
        "C8" => C8,
        /// Client Specifically Requested Consideration of Special Financing or an Assumable Loan
        "C9" => C9,
        /// Cane Required
        "CA" => Ca,
        /// Complete Bedrest
        "CB" => Cb,
        /// Collection Card was Left
        "CC" => Cc,
        /// Call to Directory Assistance for Reference Telephone
        "CD" => Cd,
        /// Co-signer Telephone Attempt
        "CE" => Ce,
        /// Co-signer Telephone Contact
        "CF" => Cf,
        /// Claim is Fraudulent
        "CFD" => Cfd,
        /// Co-signer Delinquency Letter Sent
        "CG" => Cg,
        /// Co-signer Final Demand Letter Sent
        "CH" => Ch,
        /// Call to Directory Assistance for Co-signer Telephone
        "CI" => Ci,
        /// Valid Borrower Address or Phone Attempt with Previous Holder
        "CJ" => Cj,
        /// Convertible
        "CK" => Ck,
        /// Claimant had a Pre-existing Injury
        "CL" => Cl,
        /// Comatose
        "CM" => Cm,
        /// Common Elements are Leased to or by the Home Owners' Association
        "CN" => Cn,
        /// Cumulative Injury
        "CNJ" => Cnj,
        /// Contracture
        "CO" => Co,
        /// Case Pending
        "CP" => Cp,
        /// Callable
        "CQ" => Cq,
        /// Crutches Required
        "CR" => Cr,
        /// Community Participates in National Flood Insurance Program
        "CS" => Cs,
        /// Common Elements are Completed
        "CT" => Ct,
        /// Curb and Gutter are Public
        "CU" => Cu,
        /// Cooperative
        "CV" => Cv,
        /// Cooling Water is Low
        "CW" => Cw,
        /// Certification Status
        "CX" => Cx,
        /// Car Spaces are Adequate
        "CY" => Cy,
        /// Car Spaces are Inadequate
        "CZ" => Cz,
        /// Comprehensive Coverage Will Transfer
        "D0" => D0,
        /// Issue Check Payable to Borrower and Return to Servicer
        "D1" => D1,
        /// Issue Check Payable to Servicer and Return to Servicer
        "D2" => D2,
        /// Issue Check Payable to Borrower and Send to Borrower
        "D3" => D3,
        /// Issue Check Payable to Servicer or Borrower and Return to Servicer
        "D4" => D4,
        /// Issue Check Payable to Other Payee
        "D5" => D5,
        /// Positive
        "D6" => D6,
        /// Negative
        "D7" => D7,
        /// Taxes are Typical for the Area and Price Range
        "D8" => D8,
        /// Improvement Conforms to Zoning Regulations
        "D9" => D9,
        /// Call to Directory Assistance for Borrower Telephone
        "DA" => Da,
        /// Deferment or Forbearance Begin
        "DB" => Db,
        /// Declined
        "DC" => Dc,
        /// Borrower Furnished Demographic Data
        "DD" => Dd,
        /// Deferment or Forbearance End
        "DE" => De,
        /// Funds available for Unsecured Creditors
        "DF" => Df,
        /// Deductible Amount Fully Recovered
        "DFR" => Dfr,
        /// Dynamic Brakes are Out
        "DG" => Dg,
        /// Debtor has been Domiciled
        "DH" => Dh,
        /// Disoriented
        "DI" => Di,
        /// Dynamic Brakes are Operational
        "DJ" => Dj,
        /// Construction Warranty
        "DK" => Dk,
        /// Construction Warranty Transferable
        "DL" => Dl,
        /// Maintenance Drug under Client's Benefit Plan
        "DM" => Dm,
        /// Payment Reduced Because Maximum Allowable Cost Exceeded
        "DN" => Dn,
        /// Deductible Amount Not Fully Recovered
        "DNR" => Dnr,
        /// Benefits Terminated Prior to Service Date
        "DO" => Do,
        /// Depressed
        "DP" => Dp,
        /// Drug Part of Formulary Data Base
        "DQ" => Dq,
        /// Subject not Engaged in Business
        "DR" => Dr,
        /// All Door Seals are Intact
        "DS" => Ds,
        /// Filing Fee Attached
        "DT" => Dt,
        /// Subject not Engaged in Business at Requested Address
        "DU" => Du,
        /// Suspended
        "DV" => Dv,
        /// Total
        "DW" => Dw,
        /// Unable to Respond
        "DX" => Dx,
        /// Dyspnea with Minimal Exertion
        "DY" => Dy,
        /// Uses Own Facilities
        "DZ" => Dz,
        /// Figures are Total
        "E0" => E0,
        /// Fixed Asset Breakdown Undisclosed
        "E1" => E1,
        /// For the Fiscal Year
        "E2" => E2,
        /// For the Period
        "E3" => E3,
        /// Formed by Consolidation
        "E4" => E4,
        /// Formed by Merger
        "E5" => E5,
        /// Prior Bankruptcy Case Filed in Last 6 Years
        "E6" => E6,
        /// Debtor is not Represented by an Attorney
        "E7" => E7,
        /// A Pending Case has been Filed
        "E8" => E8,
        /// Guaranteed by Parent Company
        "E9" => E9,
        /// Has Authority for All Purchases
        "EA" => Ea,
        /// Has Authority to Purchase Supplies
        "EB" => Eb,
        /// Equipment Certified
        "EC" => Ec,
        /// Has Business Interruption Insurance
        "ED" => Ed,
        /// Has Class of Stock
        "EE" => Ee,
        /// Has Extended Coverage Insurance
        "EF" => Ef,
        /// Has Fire Insurance
        "EG" => Eg,
        /// Has Joint Authority
        "EH" => Eh,
        /// Has Life Insurance
        "EI" => Ei,
        /// Existence of Preliminary Flood Determination
        "EJ" => Ej,
        /// Existence of Community Participation in the National Flood Insurance
        "EK" => Ek,
        /// Endurance Limitations
        "EL" => El,
        /// Has Marriage Contract
        "EM" => Em,
        /// Electricity On
        "EN" => En,
        /// Equipment Is Overhauled
        "EO" => Eo,
        /// Exercises Prescribed
        "EP" => Ep,
        /// Has No Par Value
        "EQ" => Eq,
        /// Engine Start-Up Performed with No Problems Reported
        "ER" => Er,
        /// Engine Start-Up Performed with Problems Reported
        "ES" => Es,
        /// Electrical Control System Shut Down
        "ET" => Et,
        /// Has Other Insurance
        "EU" => Eu,
        /// Has Par Value
        "EV" => Ev,
        /// Has Sole Authority
        "EW" => Ew,
        /// Excellent
        "EX" => Ex,
        /// Has Voting Rights
        "EY" => Ey,
        /// Heading Address in Registered Office Only
        "EZ" => Ez,
        /// High Level
        "F0" => F0,
        /// Homeworkers Employed
        "F1" => F1,
        /// In Subscriber Shares
        "F2" => F2,
        /// Inactive
        "F3" => F3,
        /// Incomplete
        "F4" => F4,
        /// Incorporation Details Requested
        "F5" => F5,
        /// Increase or Up
        "F6" => F6,
        /// Information Cannot Be Provided at This Time
        "F7" => F7,
        /// Information in Date
        "F8" => F8,
        /// Information Requires Investigation
        "F9" => F9,
        /// Actions has a Significant Environmental Effect
        "FA" => Fa,
        /// Application Includes Complete System
        "FB" => Fb,
        /// Antenna is Mounted on a Structure with an Existing Antenna
        "FC" => Fc,
        /// Notice of Construction or Alteration has been Filed
        "FD" => Fd,
        /// Applicant Wants to Monitor Frequency
        "FE" => Fe,
        /// Applicant has been Denied Government Benefits Due to Use of Drugs
        "FF" => Ff,
        /// Application is Certified
        "FG" => Fg,
        /// Application is for other Than a New Station
        "FH" => Fh,
        /// Fee Required
        "FI" => Fi,
        /// Flood Status
        "FJ" => Fj,
        /// Flood Insurance Required
        "FK" => Fk,
        /// Federal Flood Insurance is Available (Community Participates)
        "FL" => Fl,
        /// Inventory Valued Using LIFO (Last In/First Out)
        "FM" => Fm,
        /// Not Too High Level
        "FN" => Fn,
        /// Forgetful
        "FO" => Fo,
        /// Flood Certification with Life of Loan
        "FP" => Fp,
        /// Street Maintenance is Public
        "FQ" => Fq,
        /// Fair
        "FR" => Fr,
        /// Not Yet Registered
        "FS" => Fs,
        /// Obliged to File Balance Sheet
        "FT" => Ft,
        /// Official Confirmation Received
        "FU" => Fu,
        /// Old But Well Kept
        "FV" => Fv,
        /// Old Established Business
        "FW" => Fw,
        /// Operated at Break Even
        "FX" => Fx,
        /// Operates as Agent
        "FY" => Fy,
        /// Flood Zone Status
        "FZ" => Fz,
        /// Out of Business
        "G0" => G0,
        /// Outstanding Claims
        "G1" => G1,
        /// Gas On
        "G2" => G2,
        /// Hazardous Materials are Used or Produced
        "G3" => G3,
        /// Genetically Engineered Organisms are Used or Produced
        "G4" => G4,
        /// This is a Group Proposal
        "G5" => G5,
        /// Historical Sites Are Affected
        "G6" => G6,
        /// Facilities are Properly Accredited or Authorized
        "G7" => G7,
        /// Proprietary or Privileged Information will be contained in the Application
        "G8" => G8,
        /// This Project has an Actual or Potential Impact on the Environment
        "G9" => G9,
        /// Growth Rate is Fully Developed
        "GA" => Ga,
        /// Outstanding Social Security Claims
        "GB" => Gb,
        /// Outstanding Value Added Tax (VAT) Claims
        "GC" => Gc,
        /// Product Demonstration in Effect
        "GD" => Gd,
        /// Ownership Acknowledged in Signed Statement
        "GE" => Ge,
        /// Ownership Acknowledged Verbally
        "GF" => Gf,
        /// Ownership Not Acknowledged
        "GG" => Gg,
        /// Owns No Real Estate
        "GH" => Gh,
        /// Owns Real Estate but Details Not Available
        "GI" => Gi,
        /// Prepared from Books Without Audit
        "GJ" => Gj,
        /// Prepared from Statement by Accountant
        "GK" => Gk,
        /// Profits Paid to Group
        "GL" => Gl,
        /// Shelf Set to Manufacturer's Standard
        "GM" => Gm,
        /// Publicly Traded
        "GN" => Gn,
        /// Good
        "GO" => Go,
        /// Purchase Authority is Qualified
        "GP" => Gp,
        /// Purchases on Floor Plan
        "GQ" => Gq,
        /// Shelf Set to Retailer's Schematic
        "GR" => Gr,
        /// Purchases on Letter of Credit
        "GS" => Gs,
        /// Real Estate Check is Necessary
        "GT" => Gt,
        /// Record of Preferential Claims
        "GU" => Gu,
        /// Registered Address is Same as Business Address
        "GV" => Gv,
        /// Relatives Help in Business
        "GW" => Gw,
        /// Satisfactory
        "GX" => Gx,
        /// Seasons are Steady
        "GY" => Gy,
        /// Secured
        "GZ" => Gz,
        /// Organization Certifies Compliance with Federal Lobbying Regulations
        "H0" => H0,
        /// Project involves International Co-operative Activities
        "H1" => H1,
        /// Human Anatomical Substances Are Used
        "H2" => H2,
        /// Handicap Facilities Are Available
        "H3" => H3,
        /// Lobbying Activities Have Been Conducted Regarding the Proposal
        "H4" => H4,
        /// Organization Certifies Compliance With the Drug-Free Workplace Act
        "H5" => H5,
        /// Organization Certifies Compliance with the Code of Federal Regulations Regarding Research Misconduct
        "H6" => H6,
        /// Organization Provides a Smoke Free Workplace
        "H7" => H7,
        /// Organization Certifies Compliance with Federal Discrimination Regulations
        "H8" => H8,
        /// Organization Certifies Compliance with the Code of Federal Regulations Regarding Responsibility of Applicants for Promoting Objectivity in Research for which Public Health Service (PHS) Funding is Sought
        "H9" => H9,
        /// Well Maintained
        "HA" => Ha,
        /// Interest Rate Buydown
        "HB" => Hb,
        /// Heating and Cooling for the Individual Units Separately Metered
        "HC" => Hc,
        /// High Discharge
        "HD" => Hd,
        /// High Engine Water Pressure
        "HE" => He,
        /// Interest Only
        "HF" => Hf,
        /// Graduated Payment
        "HG" => Hg,
        /// Principal Balance Exceeds Maximum Negative Amortization
        "HH" => Hh,
        /// Last Change
        "HI" => Hi,
        /// Liability Released
        "HJ" => Hj,
        /// Liability Not Released
        "HK" => Hk,
        /// Hearing Limitations
        "HL" => Hl,
        /// Liability Determined by Note Holder
        "HM" => Hm,
        /// After Conversion
        "HN" => Hn,
        /// Hostile
        "HO" => Ho,
        /// After Modification
        "HP" => Hp,
        /// Balloon
        "HQ" => Hq,
        /// Capitalized Mortgage
        "HR" => Hr,
        /// Federal Wages in Effect
        "HS" => Hs,
        /// Social Security Number (SSN) Never Issued
        "HT" => Ht,
        /// Name Does Not Match Social Security Number (SSN)
        "HU" => Hu,
        /// Birthdate Does Not Match Social Security Number (SSN)
        "HV" => Hv,
        /// Impossible Social Security Number (SSN)
        "HW" => Hw,
        /// Employee is Ineligible to Work
        "HX" => Hx,
        /// Metes and Bounds
        "HY" => Hy,
        /// Consolidation, Extension, Modification of Mortgage Loan (CEM)
        "HZ" => Hz,
        /// Based on Operating Data
        "I0" => I0,
        /// Uses Outside Services
        "I1" => I1,
        /// Very High Level
        "I2" => I2,
        /// Very Small
        "I3" => I3,
        /// Voluntary Bankruptcy
        "I4" => I4,
        /// Well Balanced
        "I5" => I5,
        /// Well Regarded in Business Circles
        "I6" => I6,
        /// Organization has Delinquent Federal Debts
        "I7" => I7,
        /// Organization has been Placed on the Federal Debarment and Suspension List
        "I8" => I8,
        /// No-show Indicator
        "I9" => I9,
        /// Interest Paid in Advance
        "IA" => Ia,
        /// Interest Paid in Arrears
        "IB" => Ib,
        /// Interest Carryover
        "IC" => Ic,
        /// Sells Directly
        "ID" => Id,
        /// Sells with Agents
        "IE" => Ie,
        /// Sells with Storage
        "IF" => If,
        /// Small
        "IG" => Ig,
        /// Independent at Home
        "IH" => Ih,
        /// Some Increase
        "II" => Ii,
        /// Somewhat Declining Tendency
        "IJ" => Ij,
        /// Started Some Time Ago
        "IK" => Ik,
        /// Industry Location
        "IL" => Il,
        /// Sufficient
        "IM" => Im,
        /// Indifferent
        "IN" => In,
        /// Termination Date Set
        "IO" => Io,
        /// Injury occurred on Employer's Premises
        "IP" => Ip,
        /// Terms Include Lump Sum Payments
        "IQ" => Iq,
        /// Terms Include Progress Payments
        "IR" => Ir,
        /// Terms on Cost Plus Basis
        "IS" => Is,
        /// Terms on Fixed Fee Basis
        "IT" => It,
        /// Trade Style Registered
        "IU" => Iu,
        /// Trading Address of Sole Proprietor
        "IV" => Iv,
        /// Unchanged Situation
        "IW" => Iw,
        /// Undetermined
        "IX" => Ix,
        /// Unsatisfactory
        "IY" => Iy,
        /// Unsecured
        "IZ" => Iz,
        /// Qualifies as an Energy Efficient Home
        "J0" => J0,
        /// Military Services Barred from Recruitment Activities at the Proposing Organization's Site(s)
        "J1" => J1,
        /// Rate Negotiated
        "J2" => J2,
        /// Under Penalty of Perjury the Information is True and Correct
        "J3" => J3,
        /// Project Requires Inter-Government Review for Activities that affect State or Local Government or Possible National Security Implications
        "J4" => J4,
        /// Filing on Behalf of Debtor is Authorized
        "J5" => J5,
        /// Debtor Understands the Relief available under each Bankruptcy Chapter
        "J6" => J6,
        /// Attorney Declares that Debtor has been Informed
        "J7" => J7,
        /// Attorney has Explained the Relief available under each Bankruptcy Chapter
        "J8" => J8,
        /// There has been a Transfer of a Claim Against the Debtor by or to any Petitioner
        "J9" => J9,
        /// Third Party Originated
        "JA" => Ja,
        /// Existing Construction
        "JB" => Jb,
        /// Other Lien
        "JC" => Jc,
        /// Joint Coverage Applies
        "JCA" => Jca,
        /// Subject Lien
        "JD" => Jd,
        /// No Evidence of Property Damage Observed such as Dampness, Termites, or Structure Settlement
        "JE" => Je,
        /// Primary Underwriting System
        "JF" => Jf,
        /// Non New Parts Used
        "JG" => Jg,
        /// Pledged Loan
        "JH" => Jh,
        /// Security Delivery
        "JI" => Ji,
        /// Secondary Underwriting System
        "JJ" => Jj,
        /// Distribution is Stopped
        "JK" => Jk,
        /// Sentence was Suspended
        "JL" => Jl,
        /// Very Negative Information Exists
        "JM" => Jm,
        /// Payment Notes Exist
        "JN" => Jn,
        /// Immigrated
        "JO" => Jo,
        /// Audited with Qualifications
        "JP" => Jp,
        /// Audited
        "JQ" => Jq,
        /// Temporarily Closed
        "JR" => Jr,
        /// Partial
        "JS" => Js,
        /// Telephone Number is Unpublished
        "JT" => Jt,
        /// Telephone Number is Not in Service
        "JU" => Ju,
        /// Negative Information Exists for the Group
        "JV" => Jv,
        /// The More Important Items are Only Included
        "JW" => Jw,
        /// Interest Owned by Affiliated Company
        "JX" => Jx,
        /// Interest Owned by Subject of Inquiry
        "JY" => Jy,
        /// Qualifies as a Government Approved Condominium or Project
        "JZ" => Jz,
        /// Account Receivables Breakdown Undisclosed
        "K0" => K0,
        /// Additional Record Items Available
        "K1" => K1,
        /// Address is Qualified
        "K2" => K2,
        /// All Paid In or Issued
        "K3" => K3,
        /// Appears High
        "K4" => K4,
        /// Appears Not to Guarantee Sufficient Coverage
        "K5" => K5,
        /// Appears Sufficiently High
        "K6" => K6,
        /// Appears to Indicate a Strained Situation
        "K7" => K7,
        /// Banks with Main National Banks
        "K8" => K8,
        /// Bills Paid from Branch Office
        "K9" => K9,
        /// Bills Paid from Division Office
        "KA" => Ka,
        /// Bills Paid from Headquarters Office
        "KB" => Kb,
        /// Bond Information Available
        "KC" => Kc,
        /// Changed Accounting Date
        "KD" => Kd,
        /// Clear
        "KE" => Ke,
        /// Clear Declining Tendency
        "KF" => Kf,
        /// Clear Increase
        "KG" => Kg,
        /// Cluttered
        "KH" => Kh,
        /// Company has No Other Locations
        "KI" => Ki,
        /// Company is Branch of Foreign Entity
        "KJ" => Kj,
        /// Company is Perpetual
        "KK" => Kk,
        /// Company is Tax Exempt
        "KL" => Kl,
        /// Compared to Same Period Last Year
        "KM" => Km,
        /// Conducted at a Loss
        "KN" => Kn,
        /// Inventory Valued using FIFO (First In/First Out)
        "KO" => Ko,
        /// Large
        "KP" => Kp,
        /// Letter of Agreement Present
        "KQ" => Kq,
        /// Letter of Agreement Withdrawn
        "KR" => Kr,
        /// Letter of Liability Present
        "KS" => Ks,
        /// Letter of Liability Withdrawn
        "KT" => Kt,
        /// Location Inquired Upon is a Branch
        "KU" => Ku,
        /// Location Inquired Upon is a Branch; Headquarters is Provided
        "KV" => Kv,
        /// Location inquired upon is a Headquarters
        "KW" => Kw,
        /// Location is Foreign
        "KX" => Kx,
        /// Means Exhausted
        "KY" => Ky,
        /// Medium to Large
        "KZ" => Kz,
        /// Immunization Mandated by State Law for Employment
        "L0" => L0,
        /// General Standard of 20 Degree or .5 Diopter Sphere or Cylinder Change Met
        "L1" => L1,
        /// Replacement Due to Loss or Theft
        "L2" => L2,
        /// Replacement Due to Breakage or Damage
        "L3" => L3,
        /// Replacement Due to Patient Preference
        "L4" => L4,
        /// Replacement Due to Medical Reason
        "L5" => L5,
        /// Land Contract
        "L6" => L6,
        /// Account Current
        "L7" => L7,
        /// Very Good
        "L8" => L8,
        /// Restored
        "L9" => L9,
        /// Letter of Map Amendment or Letter of Map Revision
        "LA" => La,
        /// Legally Blind
        "LB" => Lb,
        /// Producer of Goods
        "LC" => Lc,
        /// Drawback Indicator
        "LD" => Ld,
        /// Lethargic
        "LE" => Le,
        /// Customs Rule Applicable
        "LF" => Lf,
        /// Exported Pursuant to Law Regulation or to Cancel Customs Bond
        "LG" => Lg,
        /// Country of Origin Information Applies to All Prior Shipments
        "LH" => Lh,
        /// Price Estimated
        "LI" => Li,
        /// North American Free Trade Agreement (NAFTA) Preference
        "LJ" => Lj,
        /// Kit Form
        "LK" => Lk,
        /// Lockout Effective
        "LL" => Ll,
        /// Letter of Appointment
        "LM" => Lm,
        /// Facility's Emergency Response Plan Includes Specific Actions to be Taken in Response to Accidental Releases of Regulated Substances
        "LN" => Ln,
        /// Locomotive is Isolated
        "LO" => Lo,
        /// Low Engine Oil Pressure
        "LP" => Lp,
        /// Facility had a Safety Inspection
        "LQ" => Lq,
        /// Locomotive Engine is Running
        "LR" => Lr,
        /// Lessee Signature on File
        "LS" => Ls,
        /// List Specialty in Directory
        "LSD" => Lsd,
        /// Lender or Servicer Transfer
        "LT" => Lt,
        /// Evidence of Dampness
        "LU" => Lu,
        /// Evidence of Termites
        "LV" => Lv,
        /// Evidence of Structure Settlement
        "LW" => Lw,
        /// Salvage Moved
        "LX" => Lx,
        /// Address is Former Location
        "LY" => Ly,
        /// Address is Occupied by Others
        "LZ" => Lz,
        /// Facility has an Occupational Safety and Health Act (OSHA) Star or Merit Ranking
        "M0" => M0,
        /// Data Corrected
        "M1" => M1,
        /// Servicer Record Selected
        "M2" => M2,
        /// Length of Service is 3 Months or Less
        "M3" => M3,
        /// Length of Service is 3 Months or more, and Less than 1 Year
        "M4" => M4,
        /// Length of Service is 1 Year through 5 Years
        "M5" => M5,
        /// Length of Service is more than 5 Years
        "M6" => M6,
        /// Cataract or Corneal Transplant or Other Condition such as Keratoconus
        "M7" => M7,
        /// Vision in Worse Eye Correctable to 20/40 or Better with Regular Lenses
        "M8" => M8,
        /// Contact Lenses Corrected Vision in Worse Eye to 20/40 or Better
        "M9" => M9,
        /// Major Alarm Flag Reported
        "MA" => Ma,
        /// Equipment has Modified Configuration
        "MB" => Mb,
        /// Other Mental Condition
        "MC" => Mc,
        /// Marketing Time is 4 to 6 Months
        "MD" => Md,
        /// Trend Reversed
        "ME" => Me,
        /// Microprocessor Fault
        "MF" => Mf,
        /// Mortgage Insurance Application Included
        "MG" => Mg,
        /// Mortgage Credit Report Included
        "MH" => Mh,
        /// Residential Loan Application Included
        "MI" => Mi,
        /// Real Estate Information Report Included
        "MJ" => Mj,
        /// Real Estate Title Evidence Included
        "MK" => Mk,
        /// Manually Search and List
        "ML" => Ml,
        /// Property is Occupied by Tenant (Market Rent)
        "MM" => Mm,
        /// Property is Occupied by Tenant (Regulated Rent)
        "MN" => Mn,
        /// Cooperative Project Includes or Owns Any Commercial Units
        "MO" => Mo,
        /// Units and Project Amenities are Complete
        "MP" => Mp,
        /// Eligible Trust
        "MQ" => Mq,
        /// Resale Property
        "MR" => Mr,
        /// Miscellaneous Skip-Trace Attempt
        "MS" => Ms,
        /// Photos Match Description
        "MT" => Mt,
        /// Photos Show Negative Influence
        "MU" => Mu,
        /// Exclude from Monthly Debt
        "MV" => Mv,
        /// This Broker Market Analysis is being Completed for Home Market Assistance
        "MW" => Mw,
        /// This Broker Market Analysis is being Completed for Homesale or Buyout
        "MX" => Mx,
        /// Project Type is Single Family
        "MY" => My,
        /// Project Type is Other
        "MZ" => Mz,
        /// Hospitalized over-night
        "N0" => N0,
        /// Claim Involves (a) Day(s) Away From Work
        "N1" => N1,
        /// Claim involves Restricted Work Activity Without Days Away from Work
        "N2" => N2,
        /// Strike or Lockout in Progress
        "N3" => N3,
        /// Shutdown or Layoff in Progress
        "N4" => N4,
        /// Work is Seasonal
        "N5" => N5,
        /// Natural Disaster or Adverse Weather Affecting Work
        "N6" => N6,
        /// Shorter Work Schedules or Fewer Pay Periods than Usual in Effect
        "N7" => N7,
        /// Longer Work Schedules or More Pay Periods than Usual in Effect
        "N8" => N8,
        /// Other Factors Affect Claim Frequency
        "N9" => N9,
        /// No User Available
        "NA" => Na,
        /// Neighborhood Predominately Single Family Dwellings
        "NB" => Nb,
        /// Item has Direct Numerical Control
        "NC" => Nc,
        /// Note Holder Permission Required
        "ND" => Nd,
        /// No Deductible Program
        "NDP" => Ndp,
        /// Notarized
        "NE" => Ne,
        /// New Construction
        "NF" => Nf,
        /// Mortgage Points are Customarily Paid by Seller
        "NG" => Ng,
        /// No National Flood Insurance Program map
        "NH" => Nh,
        /// Seasoned Mortgage
        "NI" => Ni,
        /// Issues are Anticipated that would Affect the Ability to Secure Financing of the Subject Property
        "NJ" => Nj,
        /// Citizenship
        "NK" => Nk,
        /// Group Disability Insurance Mandatory
        "NL" => Nl,
        /// Retail Origination
        "NM" => Nm,
        /// Answer to Referenced Question is "None"
        "NN" => Nn,
        /// Arm's Length Transaction
        "NO" => No,
        /// Certification of a Non-attorney Bankruptcy Petition Preparer
        "NP" => Np,
        /// Eligible for the Fannie Mae Neighbors Program
        "NQ" => Nq,
        /// No Restrictions
        "NR" => Nr,
        /// 401K Plan in Effect
        "NS" => Ns,
        /// Lodging Provided
        "NT" => Nt,
        /// Not Used
        "NU" => Nu,
        /// Contract Labor
        "NV" => Nv,
        /// Bonuses Paid
        "NW" => Nw,
        /// Minors Employed
        "NX" => Nx,
        /// Meets Requirements for Fannie Mae Community Seconds Program
        "NY" => Ny,
        /// Purchase is a Result of Current Employer Sponsored Relocation
        "NZ" => Nz,
        /// Teaching Major
        "O0" => O0,
        /// Multiple Unspecified Instances
        "O1" => O1,
        /// Hires Part Time Employees as Needed
        "O2" => O2,
        /// Mexican Request
        "O3" => O3,
        /// Risk Management Plan Requires Predictive Filing
        "O4" => O4,
        /// Sanitized Copy
        "O5" => O5,
        /// Site Treated, Disposed, Recycled Waste On-Site or Discharged Waste to Sewer or Publicly Owned Treatment Works
        "O6" => O6,
        /// Toxic Chemical Claimed as Trade Secret
        "O7" => O7,
        /// Under Control of Reporting Facility or Parent Company
        "O8" => O8,
        /// Weather Conditions Not Known
        "O9" => O9,
        /// Seller Provided Below Market Secondary Financing
        "OA" => Oa,
        /// Fixed Site
        "OB" => Ob,
        /// Mobile Facility
        "OC" => Oc,
        /// Transfer Authorized
        "OD" => Od,
        /// Occupational Disease
        "ODZ" => Odz,
        /// Transfer Complete
        "OE" => Oe,
        /// Commercial Driver's License Verified
        "OF" => Of,
        /// Responsibility Accepted
        "OG" => Og,
        /// Waterbody Involved
        "OH" => Oh,
        /// Charges Pending
        "OI" => Oi,
        /// Driver has Proper License Class
        "OJ" => Oj,
        /// Driver Compliant with License Restrictions
        "OK" => Ok,
        /// Other Limitation
        "OL" => Ol,
        /// Driver has Commercial Driver's License
        "OM" => Om,
        /// Driver has Medical Waiver
        "ON" => On,
        /// Own other Federal Housing Administration Property
        "OO" => Oo,
        /// Out of Range Product Temperature
        "OP" => Op,
        /// Photographs Taken
        "OQ" => Oq,
        /// Other Restrictions
        "OR" => Or,
        /// Out of Service
        "OS" => Os,
        /// Oriented
        "OT" => Ot,
        /// Police Officer at Scene
        "OU" => Ou,
        /// Overridden
        "OV" => Ov,
        /// Proposed
        "OW" => Ow,
        /// Rating is Affected
        "OX" => Ox,
        /// Veteran as Defined by the Federal Housing Administration (FHA), Veterans Administration (VA), or Department of Housing and Urban Development (HUD)
        "OY" => Oy,
        /// Liability is Contingent or has a Co-signer
        "OZ" => Oz,
        /// Terminal Degree
        "P0" => P0,
        /// Patient was Discharged from the First Facility
        "P1" => P1,
        /// Patient was Admitted to the Second Facility
        "P2" => P2,
        /// Property has a Family Room or Den
        "P3" => P3,
        /// Property has Central Air Conditioning
        "P4" => P4,
        /// Property Typical of Neighborhood
        "P5" => P5,
        /// Property Deferred Maintenance Typical of Neighborhood
        "P6" => P6,
        /// Accepting Existing Patients
        "P7" => P7,
        /// Accepting New Patients
        "P8" => P8,
        /// Property Intended to be Occupied as Primary Residence
        "P9" => P9,
        /// Paralysis
        "PA" => Pa,
        /// Phone Skip Begin
        "PB" => Pb,
        /// Plan is Attached
        "PC" => Pc,
        /// Phone Skip Resolved
        "PD" => Pd,
        /// Phone Skip Exhaust
        "PE" => Pe,
        /// Paid Outside of Closing
        "PF" => Pf,
        /// Previously Failed Board Certification
        "PFB" => Pfb,
        /// Project is Subject to Ground Rent
        "PG" => Pg,
        /// Prepayable
        "PH" => Ph,
        /// Program
        "PI" => Pi,
        /// Provider is Participating
        "PJ" => Pj,
        /// Preliminary Flood Determination
        "PK" => Pk,
        /// Provider Certification in the Taxonomy Has Been Verified
        "PL" => Pl,
        /// Project and Services Budget is Maintained
        "PM" => Pm,
        /// Atypical Physical Condition
        "PN" => Pn,
        /// Personal Property Onsite
        "PO" => Po,
        /// Property Previously Winterized
        "PP" => Pp,
        /// Liability will be Resubordinated to the Loan upon Closing
        "PQ" => Pq,
        /// Poor
        "PR" => Pr,
        /// Prior Damage
        "PRD" => Prd,
        /// Publication is Included in Sharing
        "PS" => Ps,
        /// Project is Complete
        "PT" => Pt,
        /// Not Paid
        "PU" => Pu,
        /// Property Vacant 0-5 Percent
        "PV" => Pv,
        /// Partial Weight Bearing
        "PW" => Pw,
        /// Paid by Borrower Before Closing
        "PX" => Px,
        /// Property for Sale
        "PY" => Py,
        /// Property Vacant Over 5 Percent
        "PZ" => Pz,
        /// Veteran
        "Q0" => Q0,
        /// Export Product
        "Q1" => Q1,
        /// Distilled Spirit, Beer or Wine
        "Q2" => Q2,
        /// U.S. Goods Returned
        "Q3" => Q3,
        /// Candidate for U.S. Customs Service Protest
        "Q4" => Q4,
        /// Domestic Product
        "Q5" => Q5,
        /// Prior Approval Letter and Official Orders on File
        "Q6" => Q6,
        /// Importer's Substantiating Statement and Contract are on File
        "Q7" => Q7,
        /// International Transport Movement
        "Q8" => Q8,
        /// Piece Count should be Included in the Total Packing List Quantity
        "Q9" => Q9,
        /// Shipment should be Held at the Port
        "QA" => Qa,
        /// Multiple States of Origin for this Item
        "QB" => Qb,
        /// Multiple Countries of Origin for this Item
        "QC" => Qc,
        /// Letter of Credit Restricted to a Specific Bank
        "QD" => Qd,
        /// Letter of Credit Permits Transshipment
        "QE" => Qe,
        /// Letter of Credit Covers Partial Shipments
        "QF" => Qf,
        /// Dutiable Item
        "QG" => Qg,
        /// Amounts should be Pro-rated across Line Items
        "QH" => Qh,
        /// Toxic Substance Control Act (TSCA) Certification Required
        "QI" => Qi,
        /// Visa Required for this Item
        "QJ" => Qj,
        /// Item Subject to Quotas
        "QK" => Qk,
        /// Item is a Set as Defined by the General Rules of Interpretation Section 3 (GRI3)
        "QL" => Ql,
        /// Item is a Set
        "QM" => Qm,
        /// Item is an Ensemble
        "QN" => Qn,
        /// Item is a Metal Item
        "QO" => Qo,
        /// Item is a Machine Part
        "QP" => Qp,
        /// Item is a Hazardous Item
        "QQ" => Qq,
        /// Item is Eligible under the Generalized System of Preferences (GSP)
        "QR" => Qr,
        /// Quantity to be Imported has been Approved by the Necessary Agencies
        "QS" => Qs,
        /// Filing Data is to be Withheld from Public Inspection
        "QT" => Qt,
        /// Property Type Cooperative
        "QU" => Qu,
        /// Paid by Borrower at Closing
        "QV" => Qv,
        /// Paid by Other At or Before Closing
        "QW" => Qw,
        /// Treated as a Reduction to Income
        "QX" => Qx,
        /// Does Organization Receive Income from the Sale or Lease of Tangible Personal Property, the Lease of Real Property, or the Sale of Taxable Services?
        "QY" => Qy,
        /// Is organization a contractor-retailer primarily engaged in retail sales?
        "QZ" => Qz,
        /// Exempt from Public Records Law
        "R0" => R0,
        /// Debtor Holds Claim to Real Property
        "R1" => R1,
        /// Entity Claims to Hold a Secured Interest
        "R2" => R2,
        /// Debtor has Property of the Type Specified
        "R3" => R3,
        /// Debtor Elects the State Exemption
        "R4" => R4,
        /// Debtor Elects the Federal Exemption
        "R5" => R5,
        /// Co-debtor may be Jointly Liable
        "R6" => R6,
        /// Claim is Contingent
        "R7" => R7,
        /// Claim is Unliquidated
        "R8" => R8,
        /// Claim is Disputed
        "R9" => R9,
        /// Reference Telephone Attempt
        "RA" => Ra,
        /// Debtor has No Creditors Holding Unsecured Priority Claims
        "RB" => Rb,
        /// Reference Telephone Contact
        "RC" => Rc,
        /// Rental Car Arranged
        "RCA" => Rca,
        /// Rent Delinquent
        "RD" => Rd,
        /// Claim is Subject to Setoff
        "RE" => Re,
        /// Debtor has No Executory Contracts or Unexpired Leases
        "RF" => Rf,
        /// Lease is for Nonresidential Real Property
        "RG" => Rg,
        /// Debtor has No Co-debtors
        "RH" => Rh,
        /// Debtor is Married
        "RI" => Ri,
        /// Debtor's Spouse Maintains a Separate Household
        "RJ" => Rj,
        /// Real Estate Taxes are Included
        "RK" => Rk,
        /// Property Insurance is Included
        "RL" => Rl,
        /// Debtor has No Creditors Holding Secured Claims
        "RM" => Rm,
        /// Rent Control
        "RN" => Rn,
        /// Equipment is Rebuilt
        "RO" => Ro,
        /// Individual Injured in Performance of Duty
        "RP" => Rp,
        /// Individual Injured by Third Party
        "RQ" => Rq,
        /// Quality of Management and its Enforcement of Rules and Regulations Based on General Appearances
        "RR" => Rr,
        /// Pay Continued
        "RS" => Rs,
        /// Sick Leave Taken
        "RT" => Rt,
        /// Signature on File
        "RU" => Ru,
        /// Low Refrigerant Capacity Shutdown
        "RV" => Rv,
        /// Recent Defrost
        "RW" => Rw,
        /// Rated Horsepower can be Produced
        "RX" => Rx,
        /// Foreign Military Sale
        "RY" => Ry,
        /// Waiver of Prior Notice
        "RZ" => Rz,
        /// Alternate Certification Program Participant
        "S0" => S0,
        /// Services Provided at the Second Facility were available at the First Facility
        "S1" => S1,
        /// Under Treatment
        "S2" => S2,
        /// First Time Vacant
        "S3" => S3,
        /// Adverse Easement
        "S4" => S4,
        /// Disclosure Indicator
        "S5" => S5,
        /// Atypical Off Site Improvements
        "S6" => S6,
        /// Toxic Substances
        "S7" => S7,
        /// Adverse Encroachment
        "S8" => S8,
        /// Atypical Functional Condition
        "S9" => S9,
        /// Subject Property is Currently Listed
        "SA" => Sa,
        /// Debtor is a Small Business as Defined in 11 U.S.C. Section 101
        "SB" => Sb,
        /// Special Services are Mobile Home Only
        "SC" => Sc,
        /// Special Services are Leasehold or Mobile Home or Both
        "SD" => Sd,
        /// Debtor Elects to be Considered as a Small Business Under 11 U.S.C. Section 1121(e)
        "SE" => Se,
        /// Sensor Fault
        "SF" => Sf,
        /// Street Lights are Public
        "SG" => Sg,
        /// Special Services are Leasehold or Subleasehold or Both
        "SH" => Sh,
        /// Hazardous Waste
        "SI" => Si,
        /// Pest Infestation
        "SJ" => Sj,
        /// Road Maintenance Required
        "SK" => Sk,
        /// Speech Limitations
        "SL" => Sl,
        /// Currently Serving in Military
        "SM" => Sm,
        /// Major Base Support
        "SN" => Sn,
        /// Critical Support Level Met
        "SO" => So,
        /// Street is Public
        "SP" => Sp,
        /// Specialty is Primary
        "SPP" => Spp,
        /// Specialty is Secondary
        "SPS" => Sps,
        /// Local Wages in Effect
        "SQ" => Sq,
        /// Federal Worker Displacement
        "SR" => Sr,
        /// Adverse Zoning
        "SS" => Ss,
        /// New Services Requested
        "ST" => St,
        /// Continued Services Requested
        "SU" => Su,
        /// Subrogation Open
        "SUB" => Sub,
        /// Major Corporation/High Tech
        "SV" => Sv,
        /// Sidewalk is Public
        "SW" => Sw,
        /// Collective Bargaining Agreement Sent by Mail
        "SX" => Sx,
        /// Collective Bargaining Agreement Sent by Facsimile
        "SY" => Sy,
        /// Contract
        "SZ" => Sz,
        /// Under Contract
        "T0" => T0,
        /// Road Test Performed with No Problems Reported
        "T1" => T1,
        /// Road Test Performed with Problems Reported
        "T2" => T2,
        /// Tires' Brand Match
        "T3" => T3,
        /// Real Estate Taxes are Current
        "T4" => T4,
        /// Hazard Insurance is Current
        "T5" => T5,
        /// Terminate Guarantee
        "T6" => T6,
        /// Atypical External Condition
        "T7" => T7,
        /// Subsidence (Settlement of Ground Surface Caused by Loss of Support)
        "T8" => T8,
        /// Utilities Inadequate
        "T9" => T9,
        /// Collective Bargaining Agreement Sent by Electronic Bulletin Board
        "TA" => Ta,
        /// Debtor has No Creditors Holding Unsecured Nonpriority Claims
        "TB" => Tb,
        /// Transport via Cargo Aircraft
        "TC" => Tc,
        /// Annual Leave Taken
        "TD" => Td,
        /// Item is Special Test Equipment
        "TE" => Te,
        /// Operates as Representative For Others
        "TF" => Tf,
        /// Claim Involves Work Related Death
        "TG" => Tg,
        /// Claim Does Not Involve Work Related Death, Days Away from Work, or Restricted Work Activity
        "TH" => Th,
        /// Employee Has Not Recovered to Return to Work
        "TI" => Ti,
        /// Employee Has Retired
        "TJ" => Tj,
        /// Employee Has Resigned
        "TK" => Tk,
        /// Employee is Permanently and Totally Disabled
        "TL" => Tl,
        /// Traction Motor is Cut Out
        "TM" => Tm,
        /// Atypical Quality of Construction
        "TN" => Tn,
        /// Traumatic Injury
        "TNJ" => Tnj,
        /// Atypical Remodeling
        "TO" => To,
        /// Transport via Passenger Aircraft
        "TP" => Tp,
        /// Atypical Additions
        "TQ" => Tq,
        /// Transfer to Bed, or Chair, or Both
        "TR" => Tr,
        /// Adverse Marketing Conditions in Subject Property's Neighborhood
        "TS" => Ts,
        /// Neighborhood Water Source is Public
        "TT" => Tt,
        /// Neighborhood Sewage Treatment is Public
        "TU" => Tu,
        /// Telephone Number Verified
        "TV" => Tv,
        /// Neighborhood Street is Public
        "TW" => Tw,
        /// Other Miscellaneous Adverse Characteristics
        "TX" => Tx,
        /// Subject Property's Street is Public
        "TY" => Ty,
        /// Subject Property's Sewage Treatment is Public
        "TZ" => Tz,
        /// Disability
        "U0" => U0,
        /// Minimal Change
        "U1" => U1,
        /// Neat Appearance
        "U2" => U2,
        /// Net Worth Computed after Exemptions
        "U3" => U3,
        /// Net Worth Considerably Higher
        "U4" => U4,
        /// Net Worth Higher
        "U5" => U5,
        /// No Employees
        "U6" => U6,
        /// No Employees - Business Managed by Owner
        "U7" => U7,
        /// No Employees - Business Managed by Partners
        "U8" => U8,
        /// Not Out of Business
        "U9" => U9,
        /// Uninsurable, 1316 Property
        "UA" => Ua,
        /// Conducted at a Profit
        "UB" => Ub,
        /// Contingent Debt Indicated
        "UC" => Uc,
        /// Continue
        "UD" => Ud,
        /// Contracts Obtained by Bid
        "UE" => Ue,
        /// Contracts Obtained by Negotiation
        "UF" => Uf,
        /// Converted to Holding Company
        "UG" => Ug,
        /// Cross Claim Filed
        "UH" => Uh,
        /// Declining Tendency
        "UI" => Ui,
        /// Detrimental Events in Past, Relating to Business
        "UJ" => Uj,
        /// Detrimental Events in Past, Relating to Management
        "UK" => Uk,
        /// Down or Decline or Decreased
        "UL" => Ul,
        /// Employees Include Officers
        "UM" => Um,
        /// Uncooperative
        "UN" => Un,
        /// Employees Include Owners
        "UO" => Uo,
        /// Employees Include Partners
        "UP" => Up,
        /// Employees Include Temporary Workers
        "UQ" => Uq,
        /// Employees Vary According to Needs
        "UR" => Ur,
        /// Enclosed
        "US" => Us,
        /// Up as Tolerated
        "UT" => Ut,
        /// Extent of Audit, if any, Not Indicated
        "UU" => Uu,
        /// Favorable Personal Reputation
        "UV" => Uv,
        /// Figures are Abbreviated
        "UW" => Uw,
        /// Figures are Converted to Agency Format
        "UX" => Ux,
        /// Figures are Individual
        "UY" => Uy,
        /// Figures are Restated
        "UZ" => Uz,
        /// Ultimate Parent Company Financial Statement Used
        "V0" => V0,
        /// Valid Borrower Address or Phone Attempt with School Attended
        "V1" => V1,
        /// Lender Determined Borrower Moved Out of State
        "V2" => V2,
        /// Lender Determined Borrower Moved Back into State
        "V3" => V3,
        /// Lender Determined Borrower Incarcerated
        "V4" => V4,
        /// Lender Determined Borrower No Longer Incarcerated
        "V5" => V5,
        /// Original
        "V6" => V6,
        /// True and Exact Copy
        "V7" => V7,
        /// Subject Property's Water Source is Public
        "V8" => V8,
        /// Pictures Required
        "V9" => V9,
        /// Intercompany Relations Exist
        "VA" => Va,
        /// Inventory Valued at Lower of Cost or Market
        "VB" => Vb,
        /// Inventory Valued at Other Methods
        "VC" => Vc,
        /// Operates as Sole Agent
        "VD" => Vd,
        /// Without Personal Judgment
        "VE" => Ve,
        /// Work is Subcontracted
        "VF" => Vf,
        /// Not Registered
        "VG" => Vg,
        /// Immediate Attention Required
        "VH" => Vh,
        /// Vehicle Inspection Report Completed
        "VI" => Vi,
        /// Middle to Medium
        "VJ" => Vj,
        /// Rent Control Likely
        "VK" => Vk,
        /// Furnished
        "VL" => Vl,
        /// Price Range Single Family or Planned Unit Development Not Applicable
        "VM" => Vm,
        /// Price Range Condominium Not Applicable
        "VN" => Vn,
        /// Price Range Two to Four Family Not Applicable
        "VO" => Vo,
        /// Financial Figures are Projected Based on Sales
        "VP" => Vp,
        /// Financial Figures are Projected Based on Employees
        "VQ" => Vq,
        /// Parent Company has Bankruptcy
        "VR" => Vr,
        /// Headquarters has Bankruptcy
        "VS" => Vs,
        /// Commercial Motor Vehicle was Involved in this Conviction
        "VT" => Vt,
        /// Vehicle was Declared a Total Loss
        "VTL" => Vtl,
        /// Commercial Motor Vehicle was Carrying Hazardous Materials when the Offense was Committed
        "VU" => Vu,
        /// Prepared from Internal Book Figures
        "VV" => Vv,
        /// Quantity Declined
        "VW" => Vw,
        /// Quantity Details Unknown
        "VX" => Vx,
        /// Was tax paid when purchased by seller?
        "VY" => Vy,
        /// Was item depreciable?
        "VZ" => Vz,
        /// Statement is on a Trading Trust
        "W0" => W0,
        /// New Registration
        "W1" => W1,
        /// Mailing Address Change
        "W2" => W2,
        /// Residence Address Change
        "W3" => W3,
        /// Name Change
        "W4" => W4,
        /// Party Enrollment Change
        "W5" => W5,
        /// Needs Absentee Ballot
        "W6" => W6,
        /// Would Like to be Election Day Worker
        "W7" => W7,
        /// Duplicate Registration
        "W8" => W8,
        /// Forwarded Application
        "W9" => W9,
        /// Walker Required
        "WA" => Wa,
        /// Water On
        "WB" => Wb,
        /// Application Incomplete
        "WC" => Wc,
        /// Vehicle Plate Surrendered
        "WD" => Wd,
        /// Written Notice to Note Holder
        "WE" => We,
        /// Written Notice to Borrower
        "WF" => Wf,
        /// Within Specified Time Period
        "WG" => Wg,
        /// Within Specified Range
        "WH" => Wh,
        /// Injury was Work Related
        "WI" => Wi,
        /// Dealer Pricing Authorization
        "WJ" => Wj,
        /// Summary Level Information
        "WK" => Wk,
        /// Detail Level Information
        "WL" => Wl,
        /// Non-occupant Co-borrower
        "WM" => Wm,
        /// Unit is a Studio (Efficiency)
        "WN" => Wn,
        /// Equipment in Working Order
        "WO" => Wo,
        /// To be Watched
        "WP" => Wp,
        /// Undetermined Out of Business Status
        "WQ" => Wq,
        /// Wheelchair Required
        "WR" => Wr,
        /// Balance Sheet Filed
        "WS" => Ws,
        /// Winterized Tag Observed
        "WT" => Wt,
        /// Material Safety Data Sheet
        "WU" => Wu,
        /// Accepts Credit Cards
        "WV" => Wv,
        /// All Purchases Made from Headquarters
        "WW" => Ww,
        /// Busy
        "WX" => Wx,
        /// Excessive
        "WY" => Wy,
        /// Fairly new
        "WZ" => Wz,
        /// No Employees - Business Managed by Director(s)
        "X0" => X0,
        /// Gross Weekly Amount is Estimated
        "X1" => X1,
        /// Waiting Period Disability Days are Non-consecutive
        "X2" => X2,
        /// Report Depicts Most Recent Data - Interim Period(s) Omitted
        "X3" => X3,
        /// Permanent Impairment Paid at Minimum
        "X4" => X4,
        /// Employee's Death is a Result of Work Injury or Illness
        "X5" => X5,
        /// Employee's Written Social Security Number Release is on File
        "X6" => X6,
        /// Employee's Medical Records Release Authorization is on File
        "X7" => X7,
        /// Employee Returned to Work with Pre-Injury Employer
        "X8" => X8,
        /// "Cafe" Plan in Effect
        "X9" => X9,
        /// Figures are Average
        "XA" => Xa,
        /// Imports
        "XB" => Xb,
        /// In Process of Establishing
        "XC" => Xc,
        /// Intercompany Relations Consist of Endorsements
        "XD" => Xd,
        /// Intercompany Relations Consist of Guarantees
        "XE" => Xe,
        /// Intercompany Relations Consist of Leasing Arrangements
        "XF" => Xf,
        /// Intercompany Relations Consist of Sharing Accounting
        "XG" => Xg,
        /// Intercompany Relations Consist of Sharing Facilities
        "XH" => Xh,
        /// Intercompany Relations Consist of Sharing Management
        "XI" => Xi,
        /// Intercompany Relations Consist of Sharing Personnel
        "XJ" => Xj,
        /// Interest in Other Business(es) Along with Family
        "XK" => Xk,
        /// Interest in Other Business(es) Along with Others in Reported Company
        "XL" => Xl,
        /// Inventory Valued at Company's Estimates
        "XM" => Xm,
        /// Inventory Valued at Cost
        "XN" => Xn,
        /// Inventory Valued using AVCO (Average Cost)
        "XO" => Xo,
        /// Joint Ownership
        "XP" => Xp,
        /// Leases with No Rent Payments
        "XQ" => Xq,
        /// Leases with Option to Buy
        "XR" => Xr,
        /// Leases with Token Payment
        "XS" => Xs,
        /// Limited
        "XT" => Xt,
        /// Located for Several Years
        "XU" => Xu,
        /// Located Since Opening
        "XV" => Xv,
        /// Modern
        "XW" => Xw,
        /// Non-Existent
        "XX" => Xx,
        /// Officer or Owner in Other Businesses in the Same Field
        "XY" => Xy,
        /// Operates as a Distributor for Others
        "XZ" => Xz,
        /// Insured Cooperative
        "Y0" => Y0,
        /// Worked in Industry for Several Years
        "Y1" => Y1,
        /// Aircraft Operation
        "Y2" => Y2,
        /// All Classifications on Policy Accounted For
        "Y3" => Y3,
        /// Board Provided
        "Y4" => Y4,
        /// Casual Labor
        "Y5" => Y5,
        /// Certificates on File for All Subcontractors
        "Y6" => Y6,
        /// Commissions Paid
        "Y7" => Y7,
        /// Condition or Type of Records Cause Additional Audit Time
        "Y8" => Y8,
        /// Domestic Workers Employed
        "Y9" => Y9,
        /// Operates from Residence
        "YA" => Ya,
        /// Operates under License by Others
        "YB" => Yb,
        /// Rents from Month to Month
        "YC" => Yc,
        /// Semi-modern
        "YD" => Yd,
        /// Under Construction
        "YE" => Ye,
        /// Unlimited
        "YF" => Yf,
        /// Used
        "YG" => Yg,
        /// Variable
        "YH" => Yh,
        /// Holder is a Subsidiary of Reporting Agent
        "YI" => Yi,
        /// Contact is Unchanged From Previous Report
        "YJ" => Yj,
        /// Report was Filed Last Year by This Agent
        "YK" => Yk,
        /// Party is Authorized to do Business in This State
        "YL" => Yl,
        /// Clear Decrease
        "YM" => Ym,
        /// Employees Temporarily Laid Off
        "YN" => Yn,
        /// Established in the Industry
        "YO" => Yo,
        /// Global Business
        "YP" => Yp,
        /// Information to be Followed Up
        "YQ" => Yq,
        /// Known Details are Listed
        "YR" => Yr,
        /// Land is Rented
        "YS" => Ys,
        /// Low
        "YT" => Yt,
        /// Prime Commercial Area
        "YU" => Yu,
        /// Shares with Affiliated Company(ies)
        "YV" => Yv,
        /// Slightly Higher
        "YW" => Yw,
        /// Slightly Lower
        "YX" => Yx,
        /// Stagnant
        "YY" => Yy,
        /// Territory Information is Available
        "YZ" => Yz,
        /// Subcontractors Used
        "Z0" => Z0,
        /// Insured Is a Subcontractor
        "Z1" => Z1,
        /// Insured Has Multiple Entries
        "Z2" => Z2,
        /// Insured Has Retail Operations
        "Z3" => Z3,
        /// Insured Requested Division of Payroll of Employee(s)
        "Z4" => Z4,
        /// Owner or Officer Interviewed
        "Z5" => Z5,
        /// Premium Overtime Excluded
        "Z6" => Z6,
        /// Records Reflect Proper Division of Employee(s) Payroll
        "Z7" => Z7,
        /// Records Satisfactory for Audit
        "Z8" => Z8,
        /// Relatives Employed
        "Z9" => Z9,
        /// Customer - Configuration Change is Required
        "ZA" => Za,
        /// Condition Board of Inspection and Survey (INSURV) is Mission Degrading
        "ZB" => Zb,
        /// Condition Board of Inspection and Survey (INSURV) is Maintenance Related
        "ZC" => Zc,
        /// Condition Board of Inspection and Survey (INSURV) is Safety Related
        "ZD" => Zd,
        /// Repair is Mission Essential
        "ZE" => Ze,
        /// Repair is Safety Essential
        "ZF" => Zf,
        /// Periodic Maintenance is Required
        "ZG" => Zg,
        /// Condition Board of Inspection and Survey (INSURV) Discrepancy is Corrected
        "ZH" => Zh,
        /// Progress is in Jeopardy
        "ZI" => Zi,
        /// Employee's Injury or Illness is Work Related
        "ZJ" => Zj,
        /// Final - Configuration Change is Required
        "ZK" => Zk,
        /// Final - Delivery to Shop is Required
        "ZL" => Zl,
        /// Final - Requestor Workforce will Assist
        "ZM" => Zm,
        /// Job is Level 2
        "ZN" => Zn,
        /// Preliminary - Configuration Change is Required
        "ZO" => Zo,
        /// Preliminary - Delivery to Shop is Required
        "ZP" => Zp,
        /// Preliminary - Requestor Workforce will Assist
        "ZQ" => Zq,
        /// Configuration Change is Associated with Time Meter
        "ZR" => Zr,
        /// Shop Has Lead Responsibility
        "ZS" => Zs,
        /// Estimate is Derived From Job Template
        "ZT" => Zt,
        /// Requestor Holds Technical Documentation
        "ZU" => Zu,
        /// Replacement Item
        "ZV" => Zv,
        /// Canadian Standards Association (CSA) Approved
        "ZW" => Zw,
        /// Non-convertible
        "ZX" => Zx,
        /// Underwriters Laboratory (UL) Approved
        "ZY" => Zy,
        /// Mutually Defined
        "ZZ" => Zz,
    }
);

crate::code_enum!(
    /// **1336** Insurance Type Code
    ///
    /// - Data element: 1336
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 3
    ///
    /// Insurance Type Code. Code values verified against the Stedi X12 reference.
    E1336 {
        /// Medicare Secondary Working Aged Beneficiary or Spouse with Employer Group Health Plan
        "12" => N12,
        /// Medicare Secondary End-Stage Renal Disease Beneficiary in the Mandated Coordination Period with an Employer's Group Health Plan
        "13" => N13,
        /// Medicare Secondary, No-fault Insurance including Auto is Primary
        "14" => N14,
        /// Medicare Secondary Worker's Compensation
        "15" => N15,
        /// Medicare Secondary Public Health Service (PHS)or Other Federal Agency
        "16" => N16,
        /// Medicare Secondary Black Lung
        "41" => N41,
        /// Medicare Secondary Veteran's Administration
        "42" => N42,
        /// Medicare Secondary Disabled Beneficiary Under Age 65 with Large Group Health Plan (LGHP)
        "43" => N43,
        /// Medicare Secondary, Other Liability Insurance is Primary
        "47" => N47,
        /// Auto Insurance Policy
        "AP" => Ap,
        /// Commercial
        "C1" => C1,
        /// Consolidated Omnibus Budget Reconciliation Act (COBRA)
        "CO" => Co,
        /// Medicare Conditionally Primary
        "CP" => Cp,
        /// Disability
        "D" => D,
        /// Disability Benefits
        "DB" => Db,
        /// Medicare - Point of Service (POS)
        "E" => E,
        /// Exclusive Provider Organization
        "EP" => Ep,
        /// Family or Friends
        "FF" => Ff,
        /// Group Policy
        "GP" => Gp,
        /// Health Maintenance Organization (HMO)
        "HM" => Hm,
        /// Health Maintenance Organization (HMO) - Medicare Risk
        "HN" => Hn,
        /// Special Low Income Medicare Beneficiary
        "HS" => Hs,
        /// Indemnity
        "IN" => In,
        /// Individual Policy
        "IP" => Ip,
        /// Long Term Care
        "LC" => Lc,
        /// Long Term Policy
        "LD" => Ld,
        /// Life Insurance
        "LI" => Li,
        /// Litigation
        "LT" => Lt,
        /// Medicare Part A
        "MA" => Ma,
        /// Medicare Part B
        "MB" => Mb,
        /// Medicaid
        "MC" => Mc,
        /// Medigap Part A
        "MH" => Mh,
        /// Medigap Part B
        "MI" => Mi,
        /// Medicare Primary
        "MP" => Mp,
        /// Other
        "OT" => Ot,
        /// Property Insurance - Personal
        "PE" => Pe,
        /// Personal
        "PL" => Pl,
        /// Personal Payment (Cash - No Insurance)
        "PP" => Pp,
        /// Preferred Provider Organization (PPO)
        "PR" => Pr,
        /// Point of Service (POS)
        "PS" => Ps,
        /// Qualified Medicare Beneficiary
        "QM" => Qm,
        /// Property Insurance - Real
        "RP" => Rp,
        /// Supplemental Policy
        "SP" => Sp,
        /// Tax Equity Fiscal Responsibility Act (TEFRA)
        "TF" => Tf,
        /// Multiple Options Health Plan
        "U" => U,
        /// Workers Compensation
        "WC" => Wc,
        /// Wrap Up Policy
        "WU" => Wu,
    }
);

crate::code_enum!(
    /// **1387** Rate Qualifier
    ///
    /// - Data element: 1387
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 2
    ///
    /// Rate Qualifier. Code values verified against the Stedi X12 reference.
    E1387 {
        /// Reserve Requirement Rate - Demand Deposit Account (DDA)
        "1" => N1,
        /// Analysis Earnings Credit Interest Rate Not Yet Net of Reserves
        "2" => N2,
        /// Analysis Earnings Credit Interest Rate Net of Reserves
        "2A" => N2A,
        /// Book/Ledger Balance Overdraft Interest Rate
        "3" => N3,
        /// Collected Balance Overdraft Interest Rate
        "4" => N4,
        /// Deficiency Balance Interest Rate
        "5" => N5,
        /// Treasury Tax and Loan (TT & L)
        "6" => N6,
        /// Federal Deposit Insurance Corporation (FDIC)
        "7" => N7,
        /// Overdraft Reserve Requirement Recover
        "8" => N8,
        /// Federal Funds
        "9" => N9,
        /// Prime
        "10" => N10,
        /// Mutually Defined
        "11" => N11,
        /// Reserve Requirement Rate - Non-Demand Deposit Account (Non-DDA)
        "12" => N12,
        /// Money Market (MMA)
        "13" => N13,
        /// Negotiable Order of Withdrawal Reserve (NOW)
        "14" => N14,
        /// Certificate of Deposit (CD)
        "15" => N15,
        /// Base
        "BA" => Ba,
        /// Current Factor
        "CF" => Cf,
        /// Commission
        "CM" => Cm,
        /// Coupon Rate
        "CR" => Cr,
        /// Currency Exchange
        "CX" => Cx,
        /// Demand Deposit Account Interest
        "DA" => Da,
        /// Deficiency Interest Rate
        "DI" => Di,
        /// Discount
        "DR" => Dr,
        /// Deficiency Surcharge Rate
        "DS" => Ds,
        /// Interest
        "IR" => Ir,
        /// Book/Ledger Balance Net Overdraft Interest Rate
        "NB" => Nb,
        /// Collected Balance Net Overdraft Interest Rate
        "NC" => Nc,
        /// Principal Balance Factor
        "PB" => Pb,
        /// Principal and Interest Factor
        "PF" => Pf,
        /// Rebate
        "RR" => Rr,
        /// Security
        "SR" => Sr,
        /// Standby (Letter of Credit)
        "ST" => St,
        /// Uncollected Funds Usage Interest Rate
        "UF" => Uf,
    }
);
