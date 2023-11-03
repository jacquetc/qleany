#include "passenger_list_model_from_car_passengers.h"
#include "car/car_controller.h"
#include "event_dispatcher.h"
#include "passenger/passenger_controller.h"
#include <QCoroTask>

using namespace Simple::Controller;
using namespace Simple::Presenter;

PassengerListModelFromCarPassengers::PassengerListModelFromCarPassengers(QObject *parent) : QAbstractListModel(parent)
{

    connect(EventDispatcher::instance()->car(), &CarSignals::allRelationsInvalidated, this, [this](int carId) {
        if (carId == m_carId)
        {
            return;
        }
        auto task = Car::CarController::instance()->getWithDetails(carId);
        QCoro::connect(std::move(task), this, [this, carId](auto &&carDetails) {
            if (!carDetails.isValid())
            {
                return;
            }
            QList<PassengerDTO> newPassengerList = carDetails.passengers();
            QList<int> newPassengerIdList;
            for (const auto &passenger : newPassengerList)
            {
                newPassengerIdList.append(passenger.id());
            }

            // first, add the missing passengers
            for (const auto &passenger : newPassengerList)
            {
                if (!m_passengerIdList.contains(passenger.id()))
                {
                    // add the passenger
                    int row = m_passengerList.size();
                    beginInsertRows(QModelIndex(), row, row);
                    m_passengerList.append(passenger);
                    m_passengerIdList.append(passenger.id());
                    endInsertRows();
                }
            }

            // then, remove the passengerList that are not in the new list

            for (int i = m_passengerList.size() - 1; i >= 0; --i)
            {
                if (!newPassengerIdList.contains(m_passengerList[i].id()))
                {
                    // remove the passenger
                    beginRemoveRows(QModelIndex(), i, i);
                    m_passengerList.removeAt(i);
                    m_passengerIdList.removeAt(i);
                    endRemoveRows();
                }
            }
            // then, move the current ones so the list is in the same order as the new list

            for (int i = 0; i < m_passengerList.size(); ++i)
            {
                if (m_passengerIdList[i] != newPassengerList[i].id())
                {
                    // move the passenger
                    int row = newPassengerList.indexOf(m_passengerList[i]);
                    beginMoveRows(QModelIndex(), i, i, QModelIndex(), row);
                    m_passengerList.move(i, row);
                    m_passengerIdList.move(i, row);
                    endMoveRows();
                }
            }

            // finally, update those that are in both lists if the updateDateDate has changed

            for (int i = 0; i < m_passengerList.size(); ++i)
            {
                if (m_passengerList[i].updateDate() != newPassengerList[i].updateDate())
                {
                    // update the passenger
                    m_passengerList[i] = newPassengerList[i];
                    QModelIndex topLeft = index(i, 0);
                    QModelIndex bottomRight = index(i, 0);
                    emit dataChanged(topLeft, bottomRight);
                }
            }

            return;
        });
    });

    connect(EventDispatcher::instance()->car(), &CarSignals::relationRemoved, this, [this](CarRelationDTO dto) {
        if (dto.relationField() != CarRelationDTO::RelationField::Passengers)
        {
            return;
        }

        // remove the passenger list
        QList<int> relatedIds = dto.relatedIds();

        for (int id : relatedIds)
        {
            if (!m_passengerIdList.contains(id))
            {
                continue;
            }

            int index = m_passengerIdList.indexOf(id);
            if (index != -1)
            {
                beginRemoveRows(QModelIndex(), index, index);
                m_passengerList.removeAt(index);
                m_passengerIdList.removeAt(index);
                endRemoveRows();
            }
        }
    });

    connect(EventDispatcher::instance()->car(), &CarSignals::relationInserted, this, [this](CarRelationDTO dto) {
        if (dto.id() != m_carId || dto.relationField() != CarRelationDTO::RelationField::Passengers)
        {
            return;
        }

        int position = dto.position();

        // reverse dto.relatedIds()
        QList<int> relatedIds = dto.relatedIds();
        std::reverse(relatedIds.begin(), relatedIds.end());

        // fetch passenger list from controller
        for (int passengerId : relatedIds)
        {
            Passenger::PassengerController::instance()
                ->get(passengerId)
                .then([this, passengerId, position](PassengerDTO passenger) {
                    // add passenger to this model
                    if (!m_passengerIdList.contains(passengerId))
                    {
                        beginInsertRows(QModelIndex(), position, position);
                        m_passengerList.insert(position, passenger);
                        m_passengerIdList.insert(position, passengerId);
                        endInsertRows();
                    }
                });
        }
    });

    connect(EventDispatcher::instance()->passenger(), &PassengerSignals::updated, this, [this](PassengerDTO dto) {
        for (int i = 0; i < m_passengerList.size(); ++i)
        {
            if (m_passengerIdList.at(i) == dto.id())
            {
                m_passengerList[i] = dto;
                m_passengerIdList[i] = dto.id();
                emit dataChanged(index(i), index(i));
                break;
            }
        }
    });
}

QVariant PassengerListModelFromCarPassengers::headerData(int section, Qt::Orientation orientation, int role) const
{
    return QVariant();
}

int PassengerListModelFromCarPassengers::rowCount(const QModelIndex &parent) const
{
    // For list models only the root node (an invalid parent) should return the list's size. For all
    // other (valid) parents, rowCount() should return 0 so that it does not become a tree model.
    if (parent.isValid())
        return 0;

    return m_passengerList.count();
}

QVariant PassengerListModelFromCarPassengers::data(const QModelIndex &index, int role) const
{
    if (!index.isValid())
        return QVariant();

    int row = index.row();
    if (row >= m_passengerList.size())
        return QVariant();

    const PassengerDTO &passenger = m_passengerList.at(index.row());

    if (role == Qt::DisplayRole)
    {
        return passenger.name();
    }
    if (role == Qt::EditRole)
    {
        return passenger.name();
    }

    else if (role == IdRole)
        return passenger.id();
    else if (role == UuidRole)
        return passenger.uuid();
    else if (role == CreationDateRole)
        return passenger.creationDate();
    else if (role == UpdateDateRole)
        return passenger.updateDate();
    else if (role == NameRole)
        return passenger.name();

    return QVariant();
}

Qt::ItemFlags PassengerListModelFromCarPassengers::flags(const QModelIndex &index) const
{
    if (!index.isValid())
        return Qt::NoItemFlags;

    return Qt::ItemIsEditable | QAbstractItemModel::flags(index);
}

bool PassengerListModelFromCarPassengers::setData(const QModelIndex &index, const QVariant &value, int role)
{
    if (!index.isValid())
        return false;

    int row = index.row();
    if (row >= m_passengerList.size())
        return false;

    else if (role == Qt::EditRole)
    {
        return this->setData(index, value, NameRole);
    }

    else if (role == IdRole)
    {
        if (value.canConvert<int>() == false)
        {
            qCritical() << "Cannot convert value to int";
            return false;
        }

        const PassengerDTO &passenger = m_passengerList[row];

        UpdatePassengerDTO dto;
        dto.setId(m_passengerList[row].id());
        dto.setId(value.value<int>());

        Passenger::PassengerController::instance()->update(dto).then([this, index, role](auto &&result) {
            emit dataChanged(index, index, {role});
        });

        return true;
    }
    else if (role == UuidRole)
    {
        if (value.canConvert<QUuid>() == false)
        {
            qCritical() << "Cannot convert value to QUuid";
            return false;
        }

        const PassengerDTO &passenger = m_passengerList[row];

        UpdatePassengerDTO dto;
        dto.setId(m_passengerList[row].id());
        dto.setUuid(value.value<QUuid>());

        Passenger::PassengerController::instance()->update(dto).then([this, index, role](auto &&result) {
            emit dataChanged(index, index, {role});
        });

        return true;
    }
    else if (role == CreationDateRole)
    {
        if (value.canConvert<QDateTime>() == false)
        {
            qCritical() << "Cannot convert value to QDateTime";
            return false;
        }

        const PassengerDTO &passenger = m_passengerList[row];

        UpdatePassengerDTO dto;
        dto.setId(m_passengerList[row].id());
        dto.setCreationDate(value.value<QDateTime>());

        Passenger::PassengerController::instance()->update(dto).then([this, index, role](auto &&result) {
            emit dataChanged(index, index, {role});
        });

        return true;
    }
    else if (role == UpdateDateRole)
    {
        if (value.canConvert<QDateTime>() == false)
        {
            qCritical() << "Cannot convert value to QDateTime";
            return false;
        }

        const PassengerDTO &passenger = m_passengerList[row];

        UpdatePassengerDTO dto;
        dto.setId(m_passengerList[row].id());
        dto.setUpdateDate(value.value<QDateTime>());

        Passenger::PassengerController::instance()->update(dto).then([this, index, role](auto &&result) {
            emit dataChanged(index, index, {role});
        });

        return true;
    }
    else if (role == NameRole)
    {
        if (value.canConvert<QString>() == false)
        {
            qCritical() << "Cannot convert value to QString";
            return false;
        }

        const PassengerDTO &passenger = m_passengerList[row];

        UpdatePassengerDTO dto;
        dto.setId(m_passengerList[row].id());
        dto.setName(value.value<QString>());

        Passenger::PassengerController::instance()->update(dto).then([this, index, role](auto &&result) {
            emit dataChanged(index, index, {role});
        });

        return true;
    }

    return false;
}

void PassengerListModelFromCarPassengers::populate()
{
    if (m_carId == 0)
        return;
    m_passengerList.clear();
    m_passengerIdList.clear();

    auto task = Car::CarController::instance()->getWithDetails(m_carId);
    QCoro::connect(std::move(task), this, [this](auto &&result) {
        const QList<Simple::Contracts::DTO::Passenger::PassengerDTO> passengerList = result.passengers();
        beginInsertRows(QModelIndex(), 0, passengerList.size() - 1);
        m_passengerList = passengerList;
        // fill m_passengerIdList
        for (const auto &passenger : passengerList)
        {
            m_passengerIdList.append(passenger.id());
        }

        endInsertRows();
    });
}

int PassengerListModelFromCarPassengers::carId() const
{
    return m_carId;
}

void PassengerListModelFromCarPassengers::setCarId(int newCarId)
{
    if (m_carId == newCarId)
        return;
    m_carId = newCarId;

    if (m_carId == 0)
    {
        beginResetModel();
        m_passengerList.clear();
        m_passengerIdList.clear();
        endResetModel();
    }
    else
    {
        populate();
    }
    emit carIdChanged();
}

void PassengerListModelFromCarPassengers::resetCarId()
{
    setCarId(0);
}

QHash<int, QByteArray> PassengerListModelFromCarPassengers::roleNames() const
{
    QHash<int, QByteArray> names;

    // renaming id to itemId to avoid conflict with QML's id
    names[IdRole] = "itemId";
    names[UuidRole] = "uuid";
    names[CreationDateRole] = "creationDate";
    names[UpdateDateRole] = "updateDate";
    names[NameRole] = "name";
    return names;
}