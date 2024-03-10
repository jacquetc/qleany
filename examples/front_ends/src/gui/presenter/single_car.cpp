// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "single_car.h"
#include "car/car_interactor.h"
#include "event_dispatcher.h"

using namespace FrontEnds::Interactor;
using namespace FrontEnds::Presenter;

SingleCar::SingleCar(QObject *parent)
    : QObject{parent}
{
    connect(EventDispatcher::instance()->car(), &CarSignals::removed, this, [this](QList<int> removedIds) {
        if (removedIds.contains(id())) {
            resetId();
        }
    });
    connect(EventDispatcher::instance()->car(), &CarSignals::updated, this, [this](CarDTO dto) {
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
            if (m_content != dto.content()) {
                m_content = dto.content();
                Q_EMIT contentChanged();
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
    Q_EMIT idChanged();

    // clear
    if (m_id == 0) {
        m_uuid = QUuid{};
        Q_EMIT uuidChanged();

        m_creationDate = QDateTime{};
        Q_EMIT creationDateChanged();

        m_updateDate = QDateTime{};
        Q_EMIT updateDateChanged();

        m_content = QString{};
        Q_EMIT contentChanged();

    }

    // set
    else {
        Car::CarInteractor::instance()->get(m_id).then([this](const FrontEnds::Contracts::DTO::Car::CarDTO &car) {
            if (car.isInvalid()) {
                qCritical() << Q_FUNC_INFO << "Invalid carId";
                return;
            }

            m_uuid = car.uuid();
            Q_EMIT uuidChanged();

            m_creationDate = car.creationDate();
            Q_EMIT creationDateChanged();

            m_updateDate = car.updateDate();
            Q_EMIT updateDateChanged();

            m_content = car.content();
            Q_EMIT contentChanged();
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

    UpdateCarDTO dto;
    dto.setId(id());
    dto.setUuid(newUuid);
    Car::CarInteractor::instance()->update(dto).then([this](const FrontEnds::Contracts::DTO::Car::CarDTO &car) {
        if (car.isInvalid()) {
            qCritical() << Q_FUNC_INFO << "Invalid carId";
            return;
        }
        m_uuid = car.uuid();
        Q_EMIT uuidChanged();
    });
}

QDateTime SingleCar::creationDate() const
{
    return m_creationDate;
}

void SingleCar::setCreationDate(const QDateTime &newCreationDate)
{
    if (m_creationDate == newCreationDate)
        return;

    UpdateCarDTO dto;
    dto.setId(id());
    dto.setCreationDate(newCreationDate);
    Car::CarInteractor::instance()->update(dto).then([this](const FrontEnds::Contracts::DTO::Car::CarDTO &car) {
        if (car.isInvalid()) {
            qCritical() << Q_FUNC_INFO << "Invalid carId";
            return;
        }
        m_creationDate = car.creationDate();
        Q_EMIT creationDateChanged();
    });
}

QDateTime SingleCar::updateDate() const
{
    return m_updateDate;
}

void SingleCar::setUpdateDate(const QDateTime &newUpdateDate)
{
    if (m_updateDate == newUpdateDate)
        return;

    UpdateCarDTO dto;
    dto.setId(id());
    dto.setUpdateDate(newUpdateDate);
    Car::CarInteractor::instance()->update(dto).then([this](const FrontEnds::Contracts::DTO::Car::CarDTO &car) {
        if (car.isInvalid()) {
            qCritical() << Q_FUNC_INFO << "Invalid carId";
            return;
        }
        m_updateDate = car.updateDate();
        Q_EMIT updateDateChanged();
    });
}

QString SingleCar::content() const
{
    return m_content;
}

void SingleCar::setContent(const QString &newContent)
{
    if (m_content == newContent)
        return;

    UpdateCarDTO dto;
    dto.setId(id());
    dto.setContent(newContent);
    Car::CarInteractor::instance()->update(dto).then([this](const FrontEnds::Contracts::DTO::Car::CarDTO &car) {
        if (car.isInvalid()) {
            qCritical() << Q_FUNC_INFO << "Invalid carId";
            return;
        }
        m_content = car.content();
        Q_EMIT contentChanged();
    });
}
