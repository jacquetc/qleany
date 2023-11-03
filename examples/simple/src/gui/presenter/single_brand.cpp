// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "single_brand.h"
#include "brand/brand_controller.h"
#include "event_dispatcher.h"

using namespace Simple::Controller;
using namespace Simple::Presenter;

SingleBrand::SingleBrand(QObject *parent) : QObject{parent}
{
    connect(EventDispatcher::instance()->brand(), &BrandSignals::removed, this, [this](QList<int> removedIds) {
        if (removedIds.contains(id()))
        {
            resetId();
        }
    });
    connect(EventDispatcher::instance()->brand(), &BrandSignals::updated, this, [this](BrandDTO dto) {
        if (dto.id() == id())
        {

            if (m_id != dto.id())
            {
                m_id = dto.id();
                emit idChanged();
            }
            if (m_uuid != dto.uuid())
            {
                m_uuid = dto.uuid();
                emit uuidChanged();
            }
            if (m_creationDate != dto.creationDate())
            {
                m_creationDate = dto.creationDate();
                emit creationDateChanged();
            }
            if (m_updateDate != dto.updateDate())
            {
                m_updateDate = dto.updateDate();
                emit updateDateChanged();
            }
            if (m_name != dto.name())
            {
                m_name = dto.name();
                emit nameChanged();
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
    emit idChanged();

    // clear
    if (m_id == 0)
    {

        m_uuid = QUuid();
        emit uuidChanged();

        m_creationDate = QDateTime();
        emit creationDateChanged();

        m_updateDate = QDateTime();
        emit updateDateChanged();

        m_name = QString();
        emit nameChanged();
    }

    // set
    else
    {
        Brand::BrandController::instance()->get(m_id).then(
            [this](const Simple::Contracts::DTO::Brand::BrandDTO &brand) {
                m_uuid = brand.uuid();
                emit uuidChanged();

                m_creationDate = brand.creationDate();
                emit creationDateChanged();

                m_updateDate = brand.updateDate();
                emit updateDateChanged();

                m_name = brand.name();
                emit nameChanged();
            });
    }
}

void SingleBrand::resetId()
{
    setId(0);
}

QString SingleBrand::name() const
{
    return m_name;
}

void SingleBrand::setName(const QString &newName)
{
    if (m_name == newName)
        return;
    m_name = newName;

    UpdateBrandDTO dto;
    dto.setId(id());
    dto.setName(newName);
    Brand::BrandController::instance()->update(dto);

    emit nameChanged();
}