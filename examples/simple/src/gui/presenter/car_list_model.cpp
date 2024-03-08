#include "car_list_model.h"
#include "car/car_interactor.h"
#include "event_dispatcher.h"
#include <QCoroTask>

using namespace Simple::Interactor;
using namespace Simple::Presenter;

CarListModel::CarListModel(QObject *parent) : QAbstractListModel(parent)
{
    connect(EventDispatcher::instance()->car(), &CarSignals::created, this, [this](CarDTO dto) {
        beginInsertRows(QModelIndex(), m_carList.size(), m_carList.size());
        m_carList.append(dto);
        m_carIdList.append(dto.id());
        endInsertRows();
    });

    connect(EventDispatcher::instance()->car(), &CarSignals::removed, this, [this](QList<int> ids) {
        for (int i = 0; i < ids.size(); ++i)
        {
            for (int j = 0; j < m_carList.size(); ++j)
            {
                if (m_carList.at(j).id() == ids.at(i))
                {
                    beginRemoveRows(QModelIndex(), j, j);
                    m_carList.removeAt(j);
                    m_carIdList.removeAt(j);
                    endRemoveRows();
                    break;
                }
            }
        }
    });

    connect(EventDispatcher::instance()->car(), &CarSignals::updated, this, [this](CarDTO dto) {
        for (int i = 0; i < m_carList.size(); ++i)
        {
            if (m_carList.at(i).id() == dto.id())
            {
                m_carList[i] = dto;
                m_carIdList[i] = dto.id();
                Q_EMIT dataChanged(index(i), index(i));
                break;
            }
        }
    });

    populate();
}

QVariant CarListModel::headerData(int section, Qt::Orientation orientation, int role) const
{
    return QVariant();
}

int CarListModel::rowCount(const QModelIndex &parent) const
{
    // For list models only the root node (an invalid parent) should return the list's size. For all
    // other (valid) parents, rowCount() should return 0 so that it does not become a tree model.
    if (parent.isValid())
        return 0;

    return m_carList.count();
}

QVariant CarListModel::data(const QModelIndex &index, int role) const
{
    if (!index.isValid())
        return QVariant();

    int row = index.row();
    if (row >= m_carList.size())
        return QVariant();

    const CarDTO &car = m_carList.at(index.row());

    if (role == Qt::DisplayRole)
    {
        return car.content();
    }
    if (role == Qt::EditRole)
    {
        return car.content();
    }

    else if (role == IdRole)
        return car.id();
    else if (role == UuidRole)
        return car.uuid();
    else if (role == CreationDateRole)
        return car.creationDate();
    else if (role == UpdateDateRole)
        return car.updateDate();
    else if (role == ContentRole)
        return car.content();

    return QVariant();
}

Qt::ItemFlags CarListModel::flags(const QModelIndex &index) const
{
    if (!index.isValid())
        return Qt::NoItemFlags;

    return Qt::ItemIsEditable | QAbstractItemModel::flags(index);
}

bool CarListModel::setData(const QModelIndex &index, const QVariant &value, int role)
{
    if (!index.isValid())
        return false;

    int row = index.row();
    if (row >= m_carList.size())
        return false;

    else if (role == Qt::EditRole)
    {
        return this->setData(index, value, ContentRole);
    }

    else if (role == IdRole)
    {
        if (value.canConvert<int>() == false)
        {
            qCritical() << "Cannot convert value to int";
            return false;
        }

        const CarDTO &car = m_carList[row];

        UpdateCarDTO dto;
        dto.setId(car.id());
        dto.setId(value.value<int>());

        Car::CarInteractor::instance()->update(dto).then([this, index, role](auto &&result) {
            if (result.isInvalid())
            {
                qCritical() << Q_FUNC_INFO << "Invalid ";
                return false;
            }
            Q_EMIT dataChanged(index, index, {role});
            return true;
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

        const CarDTO &car = m_carList[row];

        UpdateCarDTO dto;
        dto.setId(car.id());
        dto.setUuid(value.value<QUuid>());

        Car::CarInteractor::instance()->update(dto).then([this, index, role](auto &&result) {
            if (result.isInvalid())
            {
                qCritical() << Q_FUNC_INFO << "Invalid ";
                return false;
            }
            Q_EMIT dataChanged(index, index, {role});
            return true;
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

        const CarDTO &car = m_carList[row];

        UpdateCarDTO dto;
        dto.setId(car.id());
        dto.setCreationDate(value.value<QDateTime>());

        Car::CarInteractor::instance()->update(dto).then([this, index, role](auto &&result) {
            if (result.isInvalid())
            {
                qCritical() << Q_FUNC_INFO << "Invalid ";
                return false;
            }
            Q_EMIT dataChanged(index, index, {role});
            return true;
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

        const CarDTO &car = m_carList[row];

        UpdateCarDTO dto;
        dto.setId(car.id());
        dto.setUpdateDate(value.value<QDateTime>());

        Car::CarInteractor::instance()->update(dto).then([this, index, role](auto &&result) {
            if (result.isInvalid())
            {
                qCritical() << Q_FUNC_INFO << "Invalid ";
                return false;
            }
            Q_EMIT dataChanged(index, index, {role});
            return true;
        });

        return true;
    }
    else if (role == ContentRole)
    {
        if (value.canConvert<QString>() == false)
        {
            qCritical() << "Cannot convert value to QString";
            return false;
        }

        const CarDTO &car = m_carList[row];

        UpdateCarDTO dto;
        dto.setId(car.id());
        dto.setContent(value.value<QString>());

        Car::CarInteractor::instance()->update(dto).then([this, index, role](auto &&result) {
            if (result.isInvalid())
            {
                qCritical() << Q_FUNC_INFO << "Invalid ";
                return false;
            }
            Q_EMIT dataChanged(index, index, {role});
            return true;
        });

        return true;
    }

    return false;
}

void CarListModel::populate()
{
    beginResetModel();
    m_carList.clear();
    m_carIdList.clear();
    endResetModel();

    auto task = Car::CarInteractor::instance()->getAll();
    QCoro::connect(std::move(task), this, [this](auto &&result) {
        const QList<Simple::Contracts::DTO::Car::CarDTO> carList = result;
        for (const auto &car : carList)
        {
            if (car.isInvalid())
            {
                qCritical() << Q_FUNC_INFO << "Invalid ";
                return;
            }
        }
        if (carList.isEmpty())
        {
            return;
        }
        beginInsertRows(QModelIndex(), 0, carList.size() - 1);
        m_carList = carList;
        // fill m_carIdList
        for (const auto &car : carList)
        {
            m_carIdList.append(car.id());
        }

        endInsertRows();
    });
}

QHash<int, QByteArray> CarListModel::roleNames() const
{
    QHash<int, QByteArray> names;

    // renaming id to itemId to avoid conflict with QML's id
    names[IdRole] = "itemId";
    names[UuidRole] = "uuid";
    names[CreationDateRole] = "creationDate";
    names[UpdateDateRole] = "updateDate";
    names[ContentRole] = "content";
    return names;
}
