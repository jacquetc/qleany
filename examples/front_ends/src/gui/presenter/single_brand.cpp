// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "single_brand.h"
#include "brand/brand_controller.h"
#include "event_dispatcher.h"

using namespace FrontEnds::Controller;
using namespace FrontEnds::Presenter;

SingleBrand::SingleBrand(QObject *parent)
    : QObject{parent}
{
    connect(EventDispatcher::instance()->brand(), &BrandSignals::removed, this, [this](QList<int> removedIds) {
        if (removedIds.contains(id())) {
            resetId();
        }
    });
    connect(EventDispatcher::instance()->brand(), &BrandSignals::updated, this, [this](BrandDTO dto) {
        if (dto.id() == id()) {
            if (m_id != dto.id()) {
                m_id = dto.id();
                Q_EMIT idChanged();
            }
            if (m_uuid != dto.uuid()) {
                m_uuid = dto.uuid();
                Q_EMIT uuidChanged();
            }
            if (m_creationDate != dto.creationDate()) {
                m_creationDate = dto.creationDate();
                Q_EMIT creationDateChanged();
            }
            if (m_updateDate != dto.updateDate()) {
                m_updateDate = dto.updateDate();
                Q_EMIT updateDateChanged();
            }
            if (m_name != dto.name()) {
                m_name = dto.name();
                Q_EMIT nameChanged();
            }
        }
    });
}

int SingleBrand::id() const
{
    return m_id;
}

void SingleBrand::setId(int newId)
{
    if (m_id == newId)
        return;
    m_id = newId;
    Q_EMIT idChanged();

    // clear
    if (m_id == 0) {
        m_uuid = QUuid{};
        Q_EMIT uuidChanged();

        m_creationDate = QDateTime{};
        Q_EMIT creationDateChanged();

        m_updateDate = QDateTime{};
        Q_EMIT updateDateChanged();

        m_name = QString{};
        Q_EMIT nameChanged();

    }

    // set
    else {
        Brand::BrandController::instance()->get(m_id).then([this](const FrontEnds::Contracts::DTO::Brand::BrandDTO &brand) {
            if (brand.isInvalid()) {
                qCritical() << Q_FUNC_INFO << "Invalid brandId";
                return;
            }

            m_uuid = brand.uuid();
            Q_EMIT uuidChanged();

            m_creationDate = brand.creationDate();
            Q_EMIT creationDateChanged();

            m_updateDate = brand.updateDate();
            Q_EMIT updateDateChanged();

            m_name = brand.name();
            Q_EMIT nameChanged();
        });
    }
}

void SingleBrand::resetId()
{
    setId(0);
}

QUuid SingleBrand::uuid() const
{
    return m_uuid;
}

void SingleBrand::setUuid(const QUuid &newUuid)
{
    if (m_uuid == newUuid)
        return;

    UpdateBrandDTO dto;
    dto.setId(id());
    dto.setUuid(newUuid);
    Brand::BrandController::instance()->update(dto).then([this](const FrontEnds::Contracts::DTO::Brand::BrandDTO &brand) {
        if (brand.isInvalid()) {
            qCritical() << Q_FUNC_INFO << "Invalid brandId";
            return;
        }
        m_uuid = brand.uuid();
        Q_EMIT uuidChanged();
    });
}

QDateTime SingleBrand::creationDate() const
{
    return m_creationDate;
}

void SingleBrand::setCreationDate(const QDateTime &newCreationDate)
{
    if (m_creationDate == newCreationDate)
        return;

    UpdateBrandDTO dto;
    dto.setId(id());
    dto.setCreationDate(newCreationDate);
    Brand::BrandController::instance()->update(dto).then([this](const FrontEnds::Contracts::DTO::Brand::BrandDTO &brand) {
        if (brand.isInvalid()) {
            qCritical() << Q_FUNC_INFO << "Invalid brandId";
            return;
        }
        m_creationDate = brand.creationDate();
        Q_EMIT creationDateChanged();
    });
}

QDateTime SingleBrand::updateDate() const
{
    return m_updateDate;
}

void SingleBrand::setUpdateDate(const QDateTime &newUpdateDate)
{
    if (m_updateDate == newUpdateDate)
        return;

    UpdateBrandDTO dto;
    dto.setId(id());
    dto.setUpdateDate(newUpdateDate);
    Brand::BrandController::instance()->update(dto).then([this](const FrontEnds::Contracts::DTO::Brand::BrandDTO &brand) {
        if (brand.isInvalid()) {
            qCritical() << Q_FUNC_INFO << "Invalid brandId";
            return;
        }
        m_updateDate = brand.updateDate();
        Q_EMIT updateDateChanged();
    });
}

QString SingleBrand::name() const
{
    return m_name;
}

void SingleBrand::setName(const QString &newName)
{
    if (m_name == newName)
        return;

    UpdateBrandDTO dto;
    dto.setId(id());
    dto.setName(newName);
    Brand::BrandController::instance()->update(dto).then([this](const FrontEnds::Contracts::DTO::Brand::BrandDTO &brand) {
        if (brand.isInvalid()) {
            qCritical() << Q_FUNC_INFO << "Invalid brandId";
            return;
        }
        m_name = brand.name();
        Q_EMIT nameChanged();
    });
}
