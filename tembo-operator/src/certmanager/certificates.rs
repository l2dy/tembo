// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --derive Default certificates.cert-manager.io
// kopium version: 0.14.0

use kube::CustomResource;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default)]
#[kube(
    group = "cert-manager.io",
    version = "v1",
    kind = "Certificate",
    plural = "certificates"
)]
#[kube(namespaced)]
#[kube(status = "CertificateStatus")]
#[kube(schema = "disabled")]
pub struct CertificateSpec {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "additionalOutputFormats"
    )]
    pub additional_output_formats: Option<Vec<CertificateAdditionalOutputFormats>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "commonName"
    )]
    pub common_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dnsNames")]
    pub dns_names: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "emailAddresses"
    )]
    pub email_addresses: Option<Vec<String>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "encodeUsagesInRequest"
    )]
    pub encode_usages_in_request: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "ipAddresses"
    )]
    pub ip_addresses: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "isCA")]
    pub is_ca: Option<bool>,
    #[serde(rename = "issuerRef")]
    pub issuer_ref: CertificateIssuerRef,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keystores: Option<CertificateKeystores>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "literalSubject"
    )]
    pub literal_subject: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "privateKey"
    )]
    pub private_key: Option<CertificatePrivateKey>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "renewBefore"
    )]
    pub renew_before: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "revisionHistoryLimit"
    )]
    pub revision_history_limit: Option<i32>,
    #[serde(rename = "secretName")]
    pub secret_name: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "secretTemplate"
    )]
    pub secret_template: Option<CertificateSecretTemplate>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<CertificateSubject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uris: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usages: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct CertificateAdditionalOutputFormats {
    #[serde(rename = "type")]
    pub r#type: CertificateAdditionalOutputFormatsType,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub enum CertificateAdditionalOutputFormatsType {
    #[serde(rename = "DER")]
    Der,
    // https://cert-manager.io/docs/reference/api-docs/#cert-manager.io/v1.CertificateOutputFormatType
    #[serde(rename = "CombinedPEM")]
    #[default]
    CombinedPem,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct CertificateIssuerRef {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct CertificateKeystores {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub jks: Option<CertificateKeystoresJks>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pkcs12: Option<CertificateKeystoresPkcs12>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct CertificateKeystoresJks {
    pub create: bool,
    #[serde(rename = "passwordSecretRef")]
    pub password_secret_ref: CertificateKeystoresJksPasswordSecretRef,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct CertificateKeystoresJksPasswordSecretRef {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct CertificateKeystoresPkcs12 {
    pub create: bool,
    #[serde(rename = "passwordSecretRef")]
    pub password_secret_ref: CertificateKeystoresPkcs12PasswordSecretRef,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct CertificateKeystoresPkcs12PasswordSecretRef {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct CertificatePrivateKey {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub algorithm: Option<CertificatePrivateKeyAlgorithm>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encoding: Option<CertificatePrivateKeyEncoding>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "rotationPolicy"
    )]
    pub rotation_policy: Option<CertificatePrivateKeyRotationPolicy>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum CertificatePrivateKeyAlgorithm {
    #[serde(rename = "RSA")]
    Rsa,
    #[serde(rename = "ECDSA")]
    Ecdsa,
    Ed25519,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum CertificatePrivateKeyEncoding {
    #[serde(rename = "PKCS1")]
    Pkcs1,
    #[serde(rename = "PKCS8")]
    Pkcs8,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum CertificatePrivateKeyRotationPolicy {
    Never,
    Always,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct CertificateSecretTemplate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct CertificateSubject {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub countries: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub localities: Option<Vec<String>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "organizationalUnits"
    )]
    pub organizational_units: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organizations: Option<Vec<String>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "postalCodes"
    )]
    pub postal_codes: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provinces: Option<Vec<String>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "serialNumber"
    )]
    pub serial_number: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "streetAddresses"
    )]
    pub street_addresses: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CertificateStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<CertificateStatusConditions>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "failedIssuanceAttempts"
    )]
    pub failed_issuance_attempts: Option<i64>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "lastFailureTime"
    )]
    pub last_failure_time: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "nextPrivateKeySecretName"
    )]
    pub next_private_key_secret_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "notAfter")]
    pub not_after: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "notBefore")]
    pub not_before: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "renewalTime"
    )]
    pub renewal_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision: Option<i64>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CertificateStatusConditions {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "lastTransitionTime"
    )]
    pub last_transition_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "observedGeneration"
    )]
    pub observed_generation: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    pub status: CertificateStatusConditionsStatus,
    #[serde(rename = "type")]
    pub r#type: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum CertificateStatusConditionsStatus {
    True,
    False,
    Unknown,
}
