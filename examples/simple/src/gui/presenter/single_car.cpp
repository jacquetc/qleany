// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "single_car.h"
#include "car/car_controller.h"
#include "event_dispatcher.h"

using namespace Simple::Controller;
using namespace Simple::Presenter;

SingleCar::SingleCar(QObject *parent) : QObject{parent}
{
    connect(EventDispatcher::instance()->car(), &CarSignals::removed, this, [this](QList<int> removedIds) {
        if (removedIds.contains(id()))
        {
            resetId();
        }
    });
    connect(EventDispatcher::instance()->car(), &CarSignals::updated, this, [this](CarDTO dto) {
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
            if (m_content != dto.content())
            {
                m_content = dto.content();
                emit contentChanged();
            }
        }
    });
}

int SingleCar::id() const
{
    return m_id;
}

void SingleCar::setId(int newId)
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

        m_content = QString();
        emit contentChanged();
    }

    // set
    else
    {
        Car::CarController::instance()->get(m_id).then([this](const Simple::Contracts::DTO::Car::CarDTO &car) {
            m_uuid = car.uuid();
            emit uuidChanged();

            m_creationDate = car.creationDate();
            emit creationDateChanged();

            m_updateDate = car.updateDate();
            emit updateDateChanged();

            m_content = car.content();
            emit contentChanged();
        });
    }
}

void SingleCar::resetId()
{
    setId(0);
}

QUuid SingleCar::uuid() const
{
    return m_uuid;
}

void SingleCar::setUuid(const QUuid &newUuid)
{
    if (m_uuid == newUuid)
        return;
    m_uuid = newUuid;

    UpdateCarDTO dto;
    dto.setId(id());
    dto.setUuid(newUuid);
    Car::CarController::instance()->update(dto);

    emit uuidChanged();
}

QDateTime SingleCar::creationDate() const
{
    return m_creationDate;
}

void SingleCar::setCreationDate(const QDateTime &newCreationDate)
{
    if (m_creationDate == newCreationDate)
        return;
    m_creationDate = newCreationDate;

    UpdateCarDTO dto;
    dto.setId(id());
    dto.setCreationDate(newCreationDate);
    Car::CarController::instance()->update(dto);

    emit creationDateChanged();
}

QDateTime SingleCar::updateDate() const
{
    return m_updateDate;
}

void SingleCar::setUpdateDate(const QDateTime &newUpdateDate)
{
    if (m_updateDate == newUpdateDate)
        return;
    m_updateDate = newUpdateDate;

    UpdateCarDTO dto;
    dto.setId(id());
    dto.setUpdateDate(newUpdateDate);
    Car::CarController::instance()->update(dto);

    emit updateDateChanged();
}

QString SingleCar::content() const
{
    return m_content;
}

void SingleCar::setContent(const QString &newContent)
{
    if (m_content == newContent)
        return;
    m_content = newContent;

    UpdateCarDTO dto;
    dto.setId(id());
    dto.setContent(newContent);
    Car::CarController::instance()->update(dto);

    emit contentChanged();
}
